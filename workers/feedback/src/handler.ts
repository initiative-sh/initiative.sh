import { Octokit } from '@octokit/core'

class AppError {
  debugMessage: string
  response: Response

  constructor(
    debugMessage: string,
    message = 'An error occurred. Please try contacting us by email: support@initiative.sh',
    responseMeta: ResponseInit | null = null,
  ) {
    this.debugMessage = debugMessage
    this.response = new Response(message, responseMeta ?? { status: 400 })
    if (!this.response.headers.has('content-type')) {
      this.response.headers.set('content-type', 'text/plain')
    }
  }
}

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
  try {
    const response = await dispatchRoutes(request)
    if (!response.headers.has('content-type')) {
      response.headers.set('content-type', 'text/plain')
    }
    return response
  } catch (e) {
    if (e instanceof AppError) {
      console.log('Error:', e.debugMessage)
      return e.response
    } else if (e instanceof Error) {
      console.log('Raw error: ', e.message)
      return new AppError(e.message).response
    } else {
      console.log('Raw error: ', e)
      return new AppError(e).response
    }
  }
}

async function dispatchRoutes(request: Request): Promise<Response> {
  switch (request.url.replace(/^https?:\/\/[^/]+/, '')) {
    case '/':
      assertRequestMethod(request, 'POST')
      await assertNotRateLimited(request.headers.get('x-real-ip'))
      return handlePostIndex(await parsePostIndexRequest(request))
    case '/healthcheck':
      assertRequestMethod(request, 'GET')
      return handleHealthCheck()
  }

  return handle404(request)
}

async function handlePostIndex(postData: PostData): Promise<Response> {
  console.log(JSON.stringify(postData))

  if (postData.error !== null) {
    return await handlePostIndexErrorReport(postData)
  } else {
    return await handlePostIndexSuggestion(postData)
  }
}

async function handleHealthCheck(): Promise<Response> {
  try {
    const octokit = new Octokit({ auth: GITHUB_TOKEN })
    const githubResponse = await octokit.request('GET /repos/{owner}/{repo}', {
      owner: GITHUB_OWNER,
      repo: GITHUB_REPO,
    })

    if (githubResponse.data.full_name === `${GITHUB_OWNER}/${GITHUB_REPO}`) {
      return new Response('Health check OK\nBuild: ' + GITHUB_SHA)
    } else {
      return new Response('Health check failed\nBuild: ' + GITHUB_SHA, {
        status: 500,
      })
    }
  } catch (e) {
    return new Response(
      'Health check failed: ' + e + '\nBuild: ' + GITHUB_SHA,
      { status: 500 },
    )
  }
}

async function parsePostIndexRequest(request: Request): Promise<PostData> {
  assertContentType(request, 'application/json')

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

function handle404(request: Request): Response {
  throw new AppError('Not found: ' + request.url, 'Not found.', { status: 404 })
}

function assertContentType(request: Request, contentType: string): void {
  if (request.headers.get('content-type') !== contentType) {
    throw new AppError(
      'Unexpected content type: ' + request.headers.get('content-type'),
      'Invalid request type.',
      { status: 415 },
    )
  }
}

function assertRequestMethod(request: Request, expectedMethod: string): void {
  if (request.method !== expectedMethod) {
    throw new AppError(
      'Unexpected request method: ' + request.method,
      'Invalid request method.',
      {
        status: 405,
        headers: new Headers({
          Allow: expectedMethod,
        }),
      },
    )
  }
}

async function assertNotRateLimited(userIP: string | null): Promise<void> {
  if (userIP === null) {
    throw new AppError(
      'Received a request with no IP address.',
      'Invalid request.',
    )
  }

  const rateLimitResultRaw = await RATE_LIMIT.get(
    getRateLimitKey(userIP, 'feedback'),
    { type: 'text' },
  )

  let rateLimitResult = null
  try {
    rateLimitResult = JSON.parse(rateLimitResultRaw ? rateLimitResultRaw : '[]')
  } catch (e) {
    // Ignore it
  }

  const rateLimitNew = Array.isArray(rateLimitResult)
    ? rateLimitResult.filter((date) => date >= Date.now())
    : []

  if (rateLimitNew.length > 1) {
    throw new AppError(
      'Too many requests for IP ' + userIP,
      'Thank you for your enthusiasm! Please try submitting again later.',
      { status: 429 },
    )
  } else {
    rateLimitNew.push(Date.now() + 600 * 1000)
    await RATE_LIMIT.put(
      getRateLimitKey(userIP, 'feedback'),
      JSON.stringify(rateLimitNew),
      { expirationTtl: 600 },
    )
  }
}

function getRateLimitKey(userIP: string, resource: string): string {
  return 'ip#' + userIP + '#' + resource
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
