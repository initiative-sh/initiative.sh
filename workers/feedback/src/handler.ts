import { Octokit } from '@octokit/core'

const OWNER = 'MikkelPaulson'
const REPO = 'initiative.sh'

export async function handleRequest(request: Request): Promise<Response> {
  const userIP = request.headers.get('x-real-ip') || ''
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
    console.log('Too many requests for IP', userIP)
    return new Response(
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

  if (request.method !== 'POST') {
    console.log('Unexpected request method:', request.method)

    return new Response('Invalid request method.', {
      status: 405,
      headers: new Headers({
        Allow: 'POST',
      }),
    })
  } else if (request.headers.get('content-type') !== 'application/json') {
    console.log('Unexpected content type:', request.headers.get('content-type'))

    return new Response('JSON body expected.', {
      status: 415,
      headers: new Headers({
        Allow: 'POST',
      }),
    })
  } else {
    let requestBody = null

    try {
      requestBody = await request.json()
    } catch (e) {
      console.log('Error 0', e)
      return new Response(e, { status: 400 })
    }

    console.log('message', requestBody.message)
    console.log('error', requestBody.error)
    console.log('user-agent', request.headers.get('user-agent'))
    console.log('history:\n' + requestBody.history)

    const octokit = new Octokit({ auth: GITHUB_TOKEN })

    if (requestBody.error) {
      // Bug report - `report` command
      const errorMessage =
        'We were unable to record your error report. Please try contacting us by email: support@initiative.sh.'
      const issueLabel = 'user error report'
      const issueTitle = sanitizeTitle(requestBody.error)
      let issueComment = `Message: ${sanitizeInline(requestBody.message)}
User-agent: ${sanitizeInline(request.headers.get('user-agent'))}
IP address: ${sanitizeInline(userIP)}`

      if (requestBody.history) {
        issueComment += `

---

${sanitizeBlock(requestBody.history)}`
      }

      let issueNumber = null

      try {
        const query = `repo:${OWNER}/${REPO} is:issue is:open label:"${issueLabel}" "${issueTitle}" in:title`
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
        console.log('Error 1', e)
        return new Response(errorMessage, { status: 400 })
      }

      if (issueNumber === null) {
        try {
          const githubResponse = await octokit.request(
            'POST /repos/{owner}/{repo}/issues',
            {
              accept: 'application/vnd.github.v3+json',
              owner: OWNER,
              repo: REPO,
              title: issueTitle,
              labels: [issueLabel],
            },
          )

          issueNumber = githubResponse.data.number
          console.log(`Created issue ${issueNumber} with title ${issueTitle}`)
        } catch (e) {
          console.log('Error 2', e)
          return new Response(errorMessage, { status: 400 })
        }
      }

      try {
        await octokit.request(
          'POST /repos/{owner}/{repo}/issues/{issue_number}/comments',
          {
            accept: 'application/vnd.github.v3+json',
            owner: OWNER,
            repo: REPO,
            issue_number: issueNumber,
            body: issueComment,
          },
        )
      } catch (e) {
        console.log('Error 3', e)
        return new Response(errorMessage, { status: 400 })
      }
    } else {
      // Suggestion - `suggest` command
      const errorMessage =
        'We were unable to record your suggestion. Please try contacting us by email: support@initiative.sh.'
      const issueLabel = 'user suggestion'
      const issueTitle = sanitizeTitle(requestBody.message)
      let issueBody = `User-agent: ${sanitizeInline(
        request.headers.get('user-agent'),
      )}
IP address: ${sanitizeInline(userIP)}`

      if (requestBody.history) {
        issueBody += `

---

${sanitizeBlock(requestBody.history)}`
      }

      try {
        const githubResponse = await octokit.request(
          'POST /repos/{owner}/{repo}/issues',
          {
            accept: 'application/vnd.github.v3+json',
            body: issueBody,
            owner: OWNER,
            repo: REPO,
            title: issueTitle,
            labels: [issueLabel],
          },
        )

        console.log(
          `Created issue ${githubResponse.data.number} with title "${githubResponse.data.title}"`,
        )
      } catch (e) {
        console.log('Error 4', e)
        return new Response(errorMessage, { status: 400 })
      }
    }

    console.log('Success')
    return new Response(
      'Your ' +
        (requestBody.error ? 'error report' : 'suggestion') +
        ' has been received. Thank you!',
      { status: 200 },
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
