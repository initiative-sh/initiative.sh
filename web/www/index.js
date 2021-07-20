import * as wasm from "initiative-web";
import autoComplete from "@tarekraafat/autocomplete.js";

const promptForm = document.getElementById("prompt-form");
const prompt = document.getElementById("prompt");
const output = document.getElementById("output");

const autoCompleteJS = new autoComplete({
  data: {
    src: async (query) => wasm.autocomplete(query),
  },
  resultsList: {
    class: "autocomplete-list",
  },
  resultItem: {
    class: "autocomplete-item",
    highlight: "autocomplete-item-highlight",
    selected: "autocomplete-item-selected",
  },
  selector: "#prompt",
  submit: true,
  wrapper: false,
});

const runCommand = command => {
  output.insertAdjacentHTML(
    'beforeend',
    "\n\n> " + command + "\n\n"
  );

  output.insertAdjacentHTML(
    'beforeend',
    wasm.command(command)
  );

  prompt.value = "";
  autoCompleteJS.close();
  window.scrollBy(0, window.innerHeight);
}

promptForm.addEventListener("submit", event => {
  event.preventDefault();
  if (prompt.value !== "") {
    runCommand(prompt.value);
  }
});

promptForm.addEventListener("navigate", event => {
  prompt.value = event.detail.selection.value;
  console.log(event);
});

promptForm.addEventListener("selection", event => {
  runCommand(event.detail.selection.value);
});

// Keep the prompt focused
prompt.addEventListener("blur", event => {
  console.log("blur");
  setTimeout(() => prompt.focus(), 100);
});

window.addEventListener("click", event => prompt.focus());

prompt.focus();
