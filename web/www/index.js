import * as wasm from "initiative-web";
import autoComplete from "@tarekraafat/autocomplete.js";

const promptForm = document.getElementById("prompt-form");
const prompt = document.getElementById("prompt");
const output = document.getElementById("output");

promptForm.addEventListener("submit", event => {
  event.preventDefault();

  const input = prompt.value;

  output.insertAdjacentHTML(
    'beforeend',
    "\n\n> " + input + "\n\n"
  );

  output.insertAdjacentHTML(
    'beforeend',
    wasm.command(prompt.value)
  );

  prompt.value = "";
  window.scrollBy(0, window.innerHeight);
});

const autoCompleteJS = new autoComplete({
  selector: "#prompt",
  data: {
    src: async (query) => wasm.autocomplete(query),
  },
  submit: true,
});

prompt.focus();
