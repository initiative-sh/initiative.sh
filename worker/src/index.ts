import { AppError } from './common'
import { handleRequest as handleFeedback } from './feedback'
import { handleRequest as handleHealthCheck } from './healthcheck'

addEventListener('fetch', (event) => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request: Request): Promise<Response> {
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
  const match = request.url.match(/^https?:\/\/[^/]+(\/.*)$/)

  if (request.method === 'POST') {
    assertContentType(request, 'application/json')
  }

  if (match !== null) {
    switch (match[1]) {
      case '/feedback':
        assertRequestMethod(request, 'POST')
        return handleFeedback(request)
      case '/feedback/healthcheck':
        assertRequestMethod(request, 'GET')
        return handleHealthCheck()
    }
  }

  return handle404(request)
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
