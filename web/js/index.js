import * as wasm from "initiative-web";
import autoComplete from "@tarekraafat/autocomplete.js";
import marked from "marked";

document.body.insertAdjacentHTML(
  "beforeend",
  "<form id=\"prompt-form\"><input type=\"text\" id=\"prompt\" autocomplete=\"off\" autocorrect=\"off\" autocapitalize=\"none\"></form>"
);

const promptFormElement = document.getElementById("prompt-form");
const promptElement = document.getElementById("prompt");
const outputElement = document.getElementById("output");

const reducedMotion = (() => {
  const mediaQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
  return mediaQuery && mediaQuery.matches;
})();

marked.use({
  renderer: {
    del: (text) => `<code class="temp-link">${text}</code>`,
  },
});

const autoCompleteJS = new autoComplete({
  data: {
    src: async (query) => (await wasm.autocomplete(query)).map(a => {
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

const runCommand = async (command) => {
  output("\\> " + command + "\n\n" + await wasm.command(command));
};

const output = (text) => {
  outputElement.insertAdjacentHTML(
    "beforeend",
    marked(text)
  );

  promptElement.value = "";
  autoCompleteJS.close();
  window.scroll({
    left: 0,
    top: document.body.clientHeight,
    behavior: reducedMotion ? "auto" : "smooth",
  });
};

promptFormElement.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (promptElement.value !== "") {
    await runCommand(promptElement.value);
  }
});

promptFormElement.addEventListener("navigate", (event) => {
  promptElement.value = event.detail.selection.value.suggestion;
});

promptFormElement.addEventListener("selection", async (event) => {
  await runCommand(event.detail.selection.value.suggestion);
});

window.addEventListener("keydown", (event) => promptElement.focus());

outputElement.addEventListener("click", async (event) => {
  if (event.target.nodeName === "CODE") {
    await runCommand(event.target.innerText);
  }
});

wasm.init()
  .then((motd) => output(motd))
  .catch((err) => console.log(err));

promptElement.focus();
