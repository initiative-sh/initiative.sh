export class AppError {
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

export async function assertNotRateLimited(
  userIP: string | null,
): Promise<void> {
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
