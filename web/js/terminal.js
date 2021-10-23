import autoComplete from "@tarekraafat/autocomplete.js"
import { marked } from "marked"

function initialize(elementId, autocompleteCallback) {
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

  const terminalElement = document.getElementById(elementId)

  terminalElement.insertAdjacentHTML(
    "beforeend",
    "<form id=\"prompt-form\"><input type=\"text\" id=\"prompt\" autocomplete=\"off\" autocorrect=\"off\" autocapitalize=\"none\"></form>"
  )

  const promptFormElement = document.getElementById("prompt-form")
  const promptElement = document.getElementById("prompt")

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
            this.lexer.inlineTokens(token.text, token.tokens)
            return token
          }
        },
        renderer: function (token) {
          return `<p class="error">${this.parser.parseInline(token.tokens)}</p>`
        },
      },
    ],
  })

  let lastQuery = ""
  const autoCompleteJS = new autoComplete({
    data: {
      keys: ["suggestion"],
      src: async (query) => {
        let results = await autocompleteCallback(query)

        if (
          results.length === 1
          && results[0].suggestion.substr(0, promptElement.value.length).toLowerCase() === promptElement.value.toLowerCase()
          && query.length > lastQuery.length
        ) {
          const suggestion = results[0].suggestion
          const existingLen = promptElement.value.length
          promptElement.value = promptElement.value + suggestion.substr(promptElement.value.length)

          if (suggestion.indexOf("[") > -1) {
            promptElement.setSelectionRange(
              Math.min(existingLen, suggestion.indexOf("[")),
              suggestion.length,
            )
          } else {
            promptElement.setSelectionRange(existingLen, suggestion.length)
          }
        }

        lastQuery = query

        return results
      }
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
    query: (query) => {
      if (promptElement.selectionEnd > promptElement.selectionStart) {
        return promptElement.value.substr(0, promptElement.selectionStart)
      } else {
        return query.split("[")[0]
      }
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
        `
      },
      highlight: "autocomplete-item-highlight",
      selected: "autocomplete-item-selected",
    },
    selector: "#prompt",
    submit: true,
    wrapper: false,
  })

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
    if (event.button == 0 && mouseMoveEvents < 5) {
      if (event.target.nodeName === "CODE") {
        await runCommand(event.target.innerText)
      } else {
        promptElement.focus()
      }
    }
  })

  promptElement.focus()

  function selectBracketedExpression(command) {
    promptElement.value = command

    const match = /\[[^\]]+\]/.exec(command)
    if (match) {
      promptElement.focus()
      promptElement.setSelectionRange(
        match.index,
        match.index + match[0].length,
      )

      if (!autoCompleteJS.isOpen) {
        autoCompleteJS.start()
      }

      return true
    } else {
      promptElement.setSelectionRange(command.length, command.length)
      return false
    }
  }

  async function runCommand(command) {
    if (!selectBracketedExpression(command)) {
      let commandElement = document.createElement("div")
      commandElement.className = "command"
      commandElement.insertAdjacentText("beforeend", command)
      document.getElementById("output").insertAdjacentElement("beforeend", commandElement)

      promptElement.value = ""
      autoCompleteJS.close()

      window.scroll({
        left: 0,
        top: document.body.clientHeight,
        behavior: reducedMotion() ? "auto" : "smooth",
      })

      if (
        commandHistory.length === 0
        || command !== commandHistory[commandHistory.length - 1]
      ) {
        commandHistory.push(command)
      }
      commandHistoryIndex = -1

      terminalElement.dispatchEvent(new CustomEvent(
        "initiative.command",
        { detail: { command } }
      ))
    }
  }

  function historyEvent(event) {
    event.preventDefault()

    switch (commandHistoryIndex) {
      case -1:
        if (event.key === "ArrowUp") {
          commandHistoryIndex = commandHistory.length - 1
        }
        break
      case 0:
        if (event.key === "ArrowUp") {
          break
        }
        // fall through
      default:
        commandHistoryIndex += event.key === "ArrowUp" ? -1 : 1
        if (commandHistoryIndex < -1 || commandHistoryIndex >= commandHistory.length) {
          commandHistoryIndex = -1
        }
    }

    promptElement.value = commandHistory[commandHistoryIndex] ?? ""
  }

  function tabEvent(event) {
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
}

function reducedMotion() {
  const mediaQuery = window.matchMedia("(prefers-reduced-motion: reduce)")
  return mediaQuery && mediaQuery.matches
}

function output(text) {
  let outputBlock = document.createElement("div")
  outputBlock.className = "output-block"
  outputBlock.insertAdjacentHTML("beforeend", marked(text))
  document.getElementById("output").insertAdjacentElement("beforeend", outputBlock)

  window.scroll({
    left: 0,
    top: document.body.clientHeight,
    behavior: reducedMotion() ? "auto" : "smooth",
  })
}

export default { initialize, output }
