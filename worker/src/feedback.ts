import { Octokit } from '@octokit/core'
import { AppError, assertNotRateLimited } from './common'

class PostData {
  error: string | null
  history: string | null
  message: string
  userAgent: string
  userIP: string

  constructor(
    error: string | null,
    history: string | null,
    message: string,
    userAgent: string,
    userIP: string,
  ) {
    this.error = error
    this.history = history
    this.message = message
    this.userAgent = userAgent
    this.userIP = userIP
  }

  getBody(): string {
    let issueBody = 'Message: ' + sanitizeInline(this.message)

    if (this.error !== null) {
      issueBody += '\nError: ' + sanitizeInline(this.error)
    }

    issueBody += '\nUser-agent: ' + sanitizeInline(this.userAgent)
    issueBody += '\nUser IP: ' + sanitizeInline(this.userIP)

    if (this.history !== null) {
      issueBody += '\n\n---\n\n' + sanitizeBlock(this.history)
    }

    return issueBody
  }
}

export async function handleRequest(request: Request): Promise<Response> {
  await assertNotRateLimited(request.headers.get('x-real-ip'))
  return handlePostIndex(await parsePostIndexRequest(request))
}

async function handlePostIndex(postData: PostData): Promise<Response> {
  console.log(JSON.stringify(postData))

  if (postData.error !== null) {
    return await handlePostIndexErrorReport(postData)
  } else {
    return await handlePostIndexSuggestion(postData)
  }
}

async function parsePostIndexRequest(request: Request): Promise<PostData> {
  try {
    const requestBody = await request.json()

    if (requestBody === null || typeof requestBody.message !== 'string') {
      throw new AppError('Request missing "message" field.', 'Invalid request.')
    }

    return new PostData(
      requestBody.error ?? null,
      requestBody.history ?? null,
      requestBody.message,
      request.headers.get('user-agent') ?? 'not provided',
      request.headers.get('x-real-ip') ?? 'not provided',
    )
  } catch (e) {
    if (typeof e === 'string') {
      throw new AppError('JSON parse error: ' + e, 'Invalid request.')
    } else {
      throw e
    }
  }
}

/**
 * Bug report - `report` command
 */
async function handlePostIndexErrorReport(
  postData: PostData,
): Promise<Response> {
  const octokit = new Octokit({ auth: GITHUB_TOKEN })

  const issueLabel = 'user error report'
  const issueTitle = postData.error || 'Empty error message'

  let issueNumber = null

  try {
    const query = `repo:${GITHUB_OWNER}/${GITHUB_REPO} is:issue is:open label:"${issueLabel}" "${sanitizeTitle(
      issueTitle,
    )}" in:title`
    console.log('Searching for ' + query)

    const searchResults = await octokit.request('GET /search/issues', {
      accept: 'application/vnd.github.v3+json',
      q: query,
      sort: 'created',
      per_page: 100,
    })

    if (searchResults.data.total_count > 0) {
      for (const issue of searchResults.data.items) {
        if (issue.title === issueTitle) {
          issueNumber = issue.number
          console.log(`Matched issue ${issueNumber}: ${issue.title}`)
          break
        }
      }

      if (issueNumber === null) {
        console.log(
          `No exact matches found among ${searchResults.data.total_count} results.`,
        )
      }
    } else {
      console.log(`No matches found`)
    }
  } catch (e) {
    throw new AppError(e + ' (1)')
  }

  if (issueNumber === null) {
    try {
      const githubResponse = await octokit.request(
        'POST /repos/{owner}/{repo}/issues',
        {
          accept: 'application/vnd.github.v3+json',
          owner: GITHUB_OWNER,
          repo: GITHUB_REPO,
          title: issueTitle,
          labels: [issueLabel],
        },
      )

      issueNumber = githubResponse.data.number
      console.log(`Created issue ${issueNumber} with title ${issueTitle}`)
    } catch (e) {
      throw new AppError(e + ' (2)')
    }
  }

  try {
    await octokit.request(
      'POST /repos/{owner}/{repo}/issues/{issue_number}/comments',
      {
        accept: 'application/vnd.github.v3+json',
        owner: GITHUB_OWNER,
        repo: GITHUB_REPO,
        issue_number: issueNumber,
        body: postData.getBody(),
      },
    )
  } catch (e) {
    throw new AppError(e + ' (3)')
  }

  console.log('Success')
  return new Response('Your error report has been received. Thank you!', {
    status: 200,
  })
}

/**
 * User suggestion - `suggest` command
 */
async function handlePostIndexSuggestion(
  postData: PostData,
): Promise<Response> {
  const octokit = new Octokit({ auth: GITHUB_TOKEN })

  const issueLabel = 'user suggestion'
  const issueTitle = sanitizeTitle(postData.message)

  try {
    const githubResponse = await octokit.request(
      'POST /repos/{owner}/{repo}/issues',
      {
        accept: 'application/vnd.github.v3+json',
        body: postData.getBody(),
        owner: GITHUB_OWNER,
        repo: GITHUB_REPO,
        title: issueTitle,
        labels: [issueLabel],
      },
    )

    console.log(
      `Created issue ${githubResponse.data.number} with title "${githubResponse.data.title}"`,
    )
  } catch (e) {
    throw new AppError(e + ' (4)')
  }

  console.log('Success')
  return new Response('Your suggestion has been received. Thank you!', {
    status: 200,
  })
}

function sanitizeTitle(input: string | null): string {
  if (input) {
    return input.replaceAll(/["\\]/g, '')
  } else {
    return 'empty'
  }
}

function sanitizeInline(input: string | null): string {
  if (input) {
    return '`' + input.replaceAll('`', '') + '`'
  } else {
    return '_empty_'
  }
}

function sanitizeBlock(input: string | null): string {
  if (input) {
    return '```text\n' + input.replaceAll(/^ {0,3}`{3,}/gm, '') + '\n```'
  } else {
    return '_empty_'
  }
}
