/*
 * Copyright © 2022 Mikkel Paulson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * -----
 *
 * Note that this module uses the MIT license, whereas the rest of the
 * application uses GPL 3.0. This is a deliberate choice, because this module is
 * intended to eventually be released as a standalone npm package.
 */

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
    if (event.button === 0 && mouseMoveEvents < 5 && event.detail === 1) {
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
      selectBracketedExpression(
        autoCompleteJS.feedback.results[autoCompleteJS.cursor].value.suggestion
      )
    } else {
      const commonPrefix = autoCompleteJS.feedback.results
        .map((result) => result.value.suggestion)
        .reduce((a, b) => {
          let acc = promptElement.value
          for (let i = promptElement.value.length; i < a.length && i < b.length; i++) {
            if (a[i] == b[i]) {
              acc += a[i]
            } else if (a[i].toLowerCase() == b[i].toLowerCase()) {
              acc += a[i].toLowerCase()
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

function getThingType(element) {
  if (element && element.classList.contains('npc')) {
    return 'npc'
  } else if (element && element.classList.contains('place')) {
    return 'place'
  } else {
    return null
  }
}

function getDataUuid(element) {
  return element ? element.getAttribute('data-uuid') : null
}

function diffThingBoxes(thingA, thingB) {
  const spansA = Array.from(thingA.querySelectorAll('p span'));
  const spansB = Array.from(thingB.querySelectorAll('p span'));

  spansB.forEach((spanB, index) => {
    const spanA = spansA[index];
    if (!spanA || spanA.textContent.trim() !== spanB.textContent.trim()) {
      spanB.classList.add('changed');
    }
  });
}

function output(text) {
  const outputBlock = document.createElement("div")
  outputBlock.className = "output-block"
  const outputInnerHtml = marked(text)
  outputBlock.insertAdjacentHTML("beforeend", outputInnerHtml)

  const docOutputBlocks = document.querySelectorAll('.output-block')
  const latestOutputBlock = (docOutputBlocks.length > 0) ? docOutputBlocks[docOutputBlocks.length - 1] : null

  const latestThingBox = latestOutputBlock ? latestOutputBlock.querySelector(".thing-box") : null
  const outputThingBox = outputBlock.querySelector(".thing-box")

  console.log(latestThingBox, outputThingBox,
              getThingType(latestThingBox), getThingType(outputThingBox),
              getDataUuid(latestThingBox), getDataUuid(outputThingBox))

  if (latestThingBox && outputThingBox &&
      getThingType(latestThingBox) === getThingType(outputThingBox) &&
      getDataUuid(latestThingBox) && getDataUuid(outputThingBox) &&
      getDataUuid(latestThingBox) === getDataUuid(outputThingBox)
     ) {
    diffThingBoxes(latestThingBox, outputThingBox)
    latestThingBox.replaceWith(outputThingBox)
  } else {
    document.getElementById("output").insertAdjacentElement("beforeend", outputBlock)
  }

  window.scroll({
    left: 0,
    top: document.body.clientHeight,
    behavior: reducedMotion() ? "auto" : "smooth",
  })
}

export default { initialize, output }
