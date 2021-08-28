export async function handleRequest(request: Request): Promise<Response> {
  const user_ip = request.headers.get('x-real-ip') || ''
  const rate_limit_result_raw = await RATE_LIMIT.get(
    getRateLimitKey(user_ip, 'feedback'),
    { type: 'text' },
  )

  let rate_limit_result = null
  try {
    rate_limit_result = JSON.parse(
      rate_limit_result_raw ? rate_limit_result_raw : '[]',
    )
  } catch (e) {
    // Ignore it
  }

  const rate_limit_new = Array.isArray(rate_limit_result)
    ? rate_limit_result.filter((date) => date >= Date.now())
    : []

  if (rate_limit_new.length > 1) {
    console.log('Too many requests for IP ' + user_ip)
    return new Response('Too many requests. Please try again later.', {
      status: 429,
    })
  } else {
    rate_limit_new.push(Date.now() + 600 * 1000)
    await RATE_LIMIT.put(
      getRateLimitKey(user_ip, 'feedback'),
      JSON.stringify(rate_limit_new),
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
    return request
      .json()
      .then(
        (body) => `message: ${body.message}
error: ${body.error}
history: ${body.history}
user-agent: ${request.headers.get('user-agent')}`,
      )
      .then((message) => {
        console.log('Success:', message)
        return new Response(message, { status: 200 })
      })
      .catch((error) => {
        console.log('Error:', error)
        return new Response(error, { status: 400 })
      })
  }
}

function getRateLimitKey(user_ip: string, resource: string): string {
  return 'ip#' + user_ip + '#' + resource
}