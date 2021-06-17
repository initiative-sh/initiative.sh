import * as wasm from "initiative-web";

const promptForm = document.getElementById("prompt-form");
const prompt = document.getElementById("prompt");
const output = document.getElementById("output");

promptForm.addEventListener("submit", event => {
  event.preventDefault();

  const input = prompt.value;

  output.insertAdjacentHTML(
    'beforeend',
    "\n> " + input + "\n"
  );

  output.insertAdjacentHTML(
    'beforeend',
    wasm.command(prompt.value)
  );

  prompt.value = "";
  window.scrollBy(0, window.innerHeight);
});

prompt.focus();
