import * as wasm from "initiative-web"
import autoComplete from "@tarekraafat/autocomplete.js"
import marked from "marked"

if (window.stillLoadingTimeout) {
  clearTimeout(window.stillLoadingTimeout)
}

if (window.stillLoadingInterval) {
  clearInterval(window.stillLoadingInterval)
}

const stillLoading = document.getElementById('still-loading')
if (stillLoading) {
  stillLoading.parentNode.removeChild(stillLoading)
}

document.getElementById("container").insertAdjacentHTML(
  "beforeend",
  "<form id=\"prompt-form\"><input type=\"text\" id=\"prompt\" autocomplete=\"off\" autocorrect=\"off\" autocapitalize=\"none\"></form>"
)

const promptFormElement = document.getElementById("prompt-form")
const promptElement = document.getElementById("prompt")
const outputElement = document.getElementById("output")

const reducedMotion = (() => {
  const mediaQuery = window.matchMedia("(prefers-reduced-motion: reduce)")
  return mediaQuery && mediaQuery.matches
})()

const commandHistory = []
var commandHistoryIndex = -1

marked.use({
  renderer: {
    del: (text) => `<code class="temp-link">${text}</code>`,
    link: (href, _, text) => `<a href="${href}" target="_blank">${text}</a>`,
  },
  extensions: [
    {
      name: "error",
      level: "block",
      start: (src) => src.match(/^! /)?.index,
      tokenizer: function (src, tokens) {
        const match = /^! (.+)/.exec(src)
        if (match) {
          const token = {
            type: "error",
            raw: match[0],
            text: match[1].trim(),
            tokens: [],
          }
          this.inlineTokens(token.text, token.tokens)
          return token
        }
      },
      renderer: function (token) {
        return `<p class="error">${this.parseInline(token.tokens)}</p>`
      },
    },
  ],
})

const autoCompleteJS = new autoComplete({
  data: {
    src: async (query) => (await wasm.autocomplete(query)).map(a => {
      return {
        suggestion: a[0],
        description: a[1],
      }
    }),
    keys: ["suggestion"],
  },
  events: {
    input: {
      keydown: (event) => {
        switch (event.key) {
          case "ArrowUp":
          case "ArrowDown":
            if (autoCompleteJS.isOpen) {
              event.preventDefault()
              event.key === "ArrowUp" ? autoCompleteJS.previous() : autoCompleteJS.next()
            } else if (commandHistory.length > 0) {
              historyEvent(event)
            }
            event.stopPropagation()
            break
          case "Tab":
            event.preventDefault()
            tabEvent(event)
            break
          case "Escape":
            if (autoCompleteJS.isOpen) {
              autoCompleteJS.close()
            } else {
              promptElement.value = ""
            }

            break
        }
      },
    },
  },
  query: (input) => input.split("[")[0],
  resultsList: {
    class: "autocomplete-list",
  },
  resultItem: {
    class: "autocomplete-item",
    element: (item, data) => {
      item.innerHTML = `
      <span class="autocomplete-item-primary">${data.match}</span>
      <span class="autocomplete-item-description">${data.value.description}</span>
      `
    },
    highlight: "autocomplete-item-highlight",
    selected: "autocomplete-item-selected",
  },
  selector: "#prompt",
  submit: true,
  wrapper: false,
})

const selectBracketedExpression = (command) => {
  promptElement.value = command

  const match = /\[[^\]]+\]/.exec(command)
  if (!match) {
    return false
  }

  promptElement.focus()
  promptElement.setSelectionRange(
    match.index,
    match.index + match[0].length,
  )

  if (!autoCompleteJS.isOpen) {
    autoCompleteJS.start()
  }

  return true
}

const runCommand = async (command) => {
  if (!selectBracketedExpression(command)) {
    let commandElement = document.createElement("div")
    commandElement.className = "command"
    commandElement.insertAdjacentText("beforeend", command)
    outputElement.insertAdjacentElement("beforeend", commandElement)

    promptElement.value = ""
    autoCompleteJS.close()

    window.scroll({
      left: 0,
      top: document.body.clientHeight,
      behavior: reducedMotion ? "auto" : "smooth",
    })

    if (
      commandHistory.length === 0
      || command !== commandHistory[commandHistory.length - 1]
    ) {
      commandHistory.push(command)
    }
    commandHistoryIndex = -1

    output(await wasm.command(command))
  }
}

const historyEvent = (event) => {
  event.preventDefault()
  commandHistoryIndex += event.key === "ArrowUp" ? -1 : 1

  if (commandHistoryIndex < -1) {
    commandHistoryIndex = commandHistory.length - 1
  } else if (commandHistoryIndex >= commandHistory.length) {
    commandHistoryIndex = -1
  }

  promptElement.value = commandHistory[commandHistoryIndex] ?? ""
}

const tabEvent = (event) => {
  if (autoCompleteJS.cursor > -1) {
    console.log(autoCompleteJS.feedback.results[autoCompleteJS.cursor].value.suggestion)
    selectBracketedExpression(
      autoCompleteJS.feedback.results[autoCompleteJS.cursor].value.suggestion
    )
  } else {
    const commonPrefix = autoCompleteJS.feedback.results
      .map((result) => result.value.suggestion)
      .reduce((a, b) => {
        let acc = ""
        for (let i = 0; i < a.length && i < b.length; i++) {
          if (a[i] == b[i]) {
            acc += a[i]
          } else {
            break
          }
        }
        return acc
      })

    selectBracketedExpression(commonPrefix)
  }
  autoCompleteJS.start()
}

const output = (text) => {
  let outputBlock = document.createElement("div")
  outputBlock.className = "output-block"
  outputBlock.insertAdjacentHTML("beforeend", marked(text))
  outputElement.insertAdjacentElement("beforeend", outputBlock)

  window.scroll({
    left: 0,
    top: document.body.clientHeight,
    behavior: reducedMotion ? "auto" : "smooth",
  })
}

promptFormElement.addEventListener("submit", async (event) => {
  event.preventDefault()
  if (promptElement.value !== "") {
    await runCommand(promptElement.value)
  }
})

promptFormElement.addEventListener("navigate", (event) => {
  selectBracketedExpression(event.detail.selection.value.suggestion)
})

promptFormElement.addEventListener("selection", async (event) => {
  if (event.detail.event.type == "click") {
    await runCommand(event.detail.selection.value.suggestion)
  }
})

window.addEventListener("keydown", (event) => {
  if (!event.ctrlKey && !event.metaKey) {
    promptElement.focus()

    switch (event.key) {
      case "ArrowUp":
      case "ArrowDown":
        historyEvent(event)
        break
      case "Tab":
        tabEvent(event)
        break
    }
  }
})

let mouseMoveEvents = 0
window.addEventListener("mousedown", () => mouseMoveEvents = 0)
window.addEventListener("mousemove", () => mouseMoveEvents++)
window.addEventListener("mouseup", async (event) => {
  if (mouseMoveEvents < 5) {
    if (event.target.nodeName === "CODE") {
      await runCommand(event.target.innerText)
    } else {
      promptElement.focus()
    }
  }
})

wasm.initialize()
  .then((motd) => output(motd))
  .catch((err) => console.log(err))

promptElement.focus()
