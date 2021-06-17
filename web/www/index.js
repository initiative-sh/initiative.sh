import * as wasm from "initiative-web";

const promptForm = document.getElementById("prompt-form");
const prompt = document.getElementById("prompt");

promptForm.addEventListener("submit", event => {
  event.preventDefault();

  wasm.command(prompt.value);
  prompt.value = "";
});

prompt.focus();
