addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const { handle } = wasm_bindgen;
const instance = wasm_bindgen(wasm)

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  await instance;

  if (request.method !== "POST") {
    console.log("Unexpected request method:", request.method);

    return new Response("Invalid request method", {
      status: 405,
      headers: new Headers({
        "Allow": "POST",
      }),
    });
  } else if (request.headers.get("content-type") !== "application/json") {
    console.log("Unexpected content type:", request.headers.get("content-type"));

    return new Response("JSON body expected", {
      status: 415,
      headers: new Headers({
        "Allow": "POST",
      }),
    });
  } else {
    return request.json()
      .then((body) => handle(
        body.message,
        body.error,
        body.history,
        request.headers.get("user-agent"),
      ))
      .then((message) => {
        console.log("Success:", message);
        return new Response(message, { status: 200 });
      })
      .catch((error) => {
        console.log("Error:", error);
        return new Response(error, { status: 400 });
      });
  }
}
