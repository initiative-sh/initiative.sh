import { Octokit } from '@octokit/core'

export async function handleRequest(): Promise<Response> {
  try {
    const octokit = new Octokit({ auth: GITHUB_TOKEN })
    const githubResponse = await octokit.request('GET /repos/{owner}/{repo}', {
      owner: GITHUB_OWNER,
      repo: GITHUB_REPO,
    })

    if (githubResponse.data.full_name === `${GITHUB_OWNER}/${GITHUB_REPO}`) {
      return new Response(`Health check OK on build ${GITHUB_SHA}`)
    } else {
      return new Response(`Health check failed on build ${GITHUB_SHA}`, {
        status: 500,
      })
    }
  } catch (e) {
    return new Response(
      `Health check failed on build ${GITHUB_SHA}: ${e.message}`,
      { status: 500 },
    )
  }
}
