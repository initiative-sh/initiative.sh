import * as wasm from "initiative-web";
import autoComplete from "@tarekraafat/autocomplete.js";

document.body.insertAdjacentHTML(
  "beforeend",
  "<form id=\"prompt-form\"><input type=\"text\" id=\"prompt\"></form>"
);

const promptFormElement = document.getElementById("prompt-form");
const promptElement = document.getElementById("prompt");
const outputElement = document.getElementById("output");

const autoCompleteJS = new autoComplete({
  data: {
    src: async (query) => wasm.autocomplete(query).map(a => {
      return {
        suggestion: a[0],
        description: a[1],
      };
    }),
    keys: ["suggestion"],
  },
  resultsList: {
    class: "autocomplete-list",
  },
  resultItem: {
    class: "autocomplete-item",
    element: (item, data) => {
      item.innerHTML = `
      <span class="autocomplete-item-primary">${data.match}</span>
      <span class="autocomplete-item-description">${data.value.description}</span>
      `;
    },
    highlight: "autocomplete-item-highlight",
    selected: "autocomplete-item-selected",
  },
  selector: "#prompt",
  submit: true,
  wrapper: false,
});

const runCommand = command => {
  output("> " + command + "\n\n" + wasm.command(command));
};

const output = text => {
  outputElement.insertAdjacentHTML(
    "beforeend",
    "\n\n" + text
  );

  promptElement.value = "";
  autoCompleteJS.close();
  window.scrollBy(0, window.innerHeight);
};

promptFormElement.addEventListener("submit", event => {
  event.preventDefault();
  if (promptElement.value !== "") {
    runCommand(promptElement.value);
  }
});

promptFormElement.addEventListener("navigate", event => {
  promptElement.value = event.detail.selection.value.suggestion;
});

promptFormElement.addEventListener("selection", event => {
  runCommand(event.detail.selection.value.suggestion);
});

// Keep the prompt focused
promptElement.addEventListener("blur", event => setTimeout(() => promptElement.focus(), 100));

window.addEventListener("click", event => promptElement.focus());

output(wasm.motd());

promptElement.focus();
