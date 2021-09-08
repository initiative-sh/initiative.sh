import { Octokit } from '@octokit/core'

export async function handleRequest(): Promise<Response> {
  const messages: string[] = []
  let ok = true

  try {
    const octokit = new Octokit({ auth: GITHUB_TOKEN })
    const githubResponse = await octokit.request('GET /repos/{owner}/{repo}', {
      owner: GITHUB_OWNER,
      repo: GITHUB_REPO,
    })

    if (githubResponse.data.full_name === `${GITHUB_OWNER}/${GITHUB_REPO}`) {
      messages.push('A: OK')
    } else {
      messages.push('A: failed')
      ok = false
    }
  } catch (e) {
    messages.push(`A: error (${e.message})`)
    ok = false
  }

  try {
    const response = await RATE_LIMIT.list()
    messages.push(`B: OK (${response.keys.length})`)
  } catch (e) {
    messages.push(`B: error (${e.message})`)
    ok = false
  }

  if (ok) {
    messages.unshift(`Health check OK on build ${GITHUB_SHA}`, '')
    return new Response(messages.join('\n'))
  } else {
    messages.unshift(`Health check failed on build ${GITHUB_SHA}`, '')
    return new Response(messages.join('\n'), { status: 500 })
  }
}
