// constants
const LS_KEY = "cli-page-links";
const LS_ENGINE_KEY = "cli-page-engine";
// File Constants
const types = {
    LINK: "link",
    DIR: "directory",
};
// Defined Engines
const ENGINES = {
    google: "https://google.com/search?q=",
    ddg: "https://duckduckgo.com/?q=",
    bing: "https://www.bing.com/search?q=",
};

/*
 * Utils
*/

function safeParse(input) {
    try {
        return JSON.parse(input) || {};
    } catch {
        return {};
    }
}

function locationType(val) {
    if (typeof val === "string") return types.LINK;
    return types.DIR;
}

function joinWriter(command, writer) {
    return (input) => {
        writer(command(input));
    };
}

function focusPrompt() {
    document.getElementById("prompt-input").focus();
}

function getCursor(pos) {
    let cursor = links;
    pos.forEach((p) => {
        cursor = cursor[p];
    });
    return cursor;
}

function getCurrentCursor() {
    return getCursor(position);
}

function pushCommand(cmd) {
    const prompt = document.getElementById("prompt-input");
    prompt.value = cmd;
    focusPrompt();
}

function locatePath(path) {
    let cursor = locateParentPath(path);
    if (path.length) {
        const final = path[path.length - 1];
        if (!cursor[final]) {
            throw `no such link or directory: ${final}`;
        }
        return cursor[final];
    }
    return cursor;
}

function locateParentPath(fullPath) {
    const path = fullPath.slice(0, fullPath.length - 1);
    let cursor = getCurrentCursor();
    const newPosition = [...position];
    for (let i = 0; i < path.length; i++) {
        const m = path[i];
        if (m === "..") {
            newPosition.pop();
            cursor = getCursor(newPosition);
        } else {
            if (!cursor[m]) {
                throw `no such link or directory: ${m}`;
            }
            if (locationType(cursor[m]) === types.LINK) {
                throw `not a directory: ${m}`;
            }
            newPosition.push(m);
            cursor = getCursor(newPosition);
        }
    }
    return cursor;
}

function replacePrompt() {
    const oldPrompt = document.getElementById("prompt-input");
    const value = oldPrompt.value;
    const promptText = document.createElement("p");
    promptText.innerText = value;
    promptText.classList.add("prompt-text");
    oldPrompt.replaceWith(promptText);
}

// Parse command input by keeping strings in "" together as an single item
function parseCommand(input) {
    const re = /"([^"]+)"|([^\s]+)/g;
    const parsedCmd = [];
    let temp;
    while ((temp = re.exec(input)) !== null) {
        const val = temp[1] || temp[2]; // Get the correct capture group
        parsedCmd.push(val);
    }
    return parsedCmd;
}

// Parse command array to extract flags
function extractFlags(command, flagMap = {}) {
    const finalCommand = [];
    const flags = {};
    for (let i = 0; i < command.length; i++) {
        const arg = command[i];
        const isFlag = /^(-|--)(\w+)/.exec(arg);
        if (isFlag) {
            const flag = isFlag[2];
            // If flag marked boolean, don't capture input
            // TODO: throw error if not found in map?
            if (flagMap[flag] !== "boolean") {
                flags[flag] = command[i + 1];
                i++;
            } else {
                flags[flag] = true;
            }
        } else {
            finalCommand.push(arg);
        }
    }
    return { command: finalCommand, flags };
}

function formatUrl(url) {
    let finalUrl = url;
    if (!/^http|https:\/\//.test(finalUrl)) {
        finalUrl = "https://" + finalUrl;
    }
    return finalUrl;
}

// LocalStorage Interaction Functions
function readLinks() {
    return safeParse(localStorage.getItem(LS_KEY));
}

function writeLinks() {
    localStorage.setItem(LS_KEY, JSON.stringify(links));
}

function readEngine() {
    return localStorage.getItem(LS_ENGINE_KEY);
}

function writeEngine(url) {
    localStorage.setItem(LS_ENGINE_KEY, url);
}

// writers
//
function listWriter(output) {
  if (Array.isArray(output)) {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<ul class='ls-links'>";
    inner =
      inner +
      output
          .map((item) => `<li class="ls-item"><a target='_blank' href='${links[item.key]}'>${item.key}</a></li>`)
        .join("");
    inner = inner + "</ul>";
    outputNode.innerHTML = inner;
    document.getElementById("links").innerHTML = "";
    terminal.appendChild(outputNode);
  } else {
    textWriter(output);
  }
}

function textWriter(output = "") {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("links");
  const textNode = document.createElement("p");
  textNode.innerText = output;
}

function buildNestedList(cursor, list) {
  Object.entries(cursor).map(([key, value]) => {
    if (locationType(value) === types.DIR) {
      list.push(
        `<li class="tree-list-item directory">${key}<ul class="tree-list">`
      );
      buildNestedList(value, list);
      list.push("</ul></li>");
    } else {
      list.push(`<li class="tree-list-item">${key}</li>`);
    }
  });
  return list;
}

function treeWriter(output = "") {
  if (Array.isArray(output)) {
    listWriter(output);
  } else if (typeof output === "object") {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("nav");
    outputNode.classList.add("links");
    let inner = "<ul class='tree-list'>";
    inner = inner + buildNestedList(output, []).join("") + "</ul>";
    outputNode.innerHTML = inner;
    terminal.appendChild(outputNode);
  } else {
    textWriter(output);
  }
}
/*
 * Commands
*/

// Add flag on ls to show actual links with names
function list(input) {
    const cursor = getCurrentCursor();
    if (locationType(cursor) === types.DIR) {
      return Object.entries(cursor).map(([key, value]) => {
        return {
          key,
          type: locationType(value), // Determine if dir or link
        };
      });
    }
}

function openLink(input) {
  if (input.length) {
    try {
      const path = input[0].split("/");
      const target = locatePath(path);
      if (locationType(target) === types.DIR) {
        return `not a link: ${path[path.length - 1]}`;
      }
      window.open(target, "_blank");
      return;
    } catch (err) {
      return err;
    }
  }
  return COMMANDS.open.help;
}

function touch(input) {
  if (input.length == 2) {
    try {
      const path = input[0].split("/");
      const url = formatUrl(input[1]);
      const parent = locateParentPath(path);
      const target = path[path.length - 1];
      parent[target] = url;
      return writeLinks();
    } catch (err) {
      return err;
    }
  }
}

function rm(input) {
  if (input.length) {
    const path = input[0].split("/");
    try {
      const parent = locateParentPath(path);
      const target = path[path.length - 1];
      if (!parent[target]) {
        return `no such link: ${target}`;
      }
      if (locationType(parent[target]) !== types.LINK) {
        return `not a link: ${target}`;
      }
      delete parent[target];
      writeLinks();
      return;
    } catch (err) {
      return err;
    }
  }
  return COMMANDS.rm.help;
}

function search(input) {
  const { command, flags } = extractFlags(input, {
    e: "string",
  });
  let currentSearchUrl = searchUrl;
  if (flags.e) {
    currentSearchUrl = ENGINES[flags.e] ? ENGINES[flags.e] : flags.e;
    if (!command[0]) {
      // Set saved engine to given
      searchUrl = currentSearchUrl;
      writeEngine(currentSearchUrl);
      return `Updated search engine to: ${currentSearchUrl}`;
    }
  }
  if (command && command[0]) {
    const searchString = command[0];
    window.open(currentSearchUrl + searchString, "_blank");
    return;
  }
  return COMMANDS.search.help;
}

/*
 * Cli
*/
// available commands
const COMMANDS = {
    ls: {
        func: joinWriter(list, treeWriter)
    },
    open: {
        func: joinWriter(openLink, textWriter)
    },
    add: {
        func: joinWriter(touch, textWriter)
    },
    del: {
        func: joinWriter(rm, textWriter)
    },
    search: {
        func: joinWriter(search, textWriter)
    },
};

let searchUrl = ENGINES.ddg; // default search engine
let links = {};
let position = []; // Determines where in the link tree we are currently

function handleKeyPresses(e) {
    if (e.keyCode === 13) {
        // Enter
        const input = document.getElementById("prompt-input");
        return runCommand(input.value);
        focusPrompt()
    }
}

// user commands
function runCommand(cmd) {
    const parsedCmd = parseCommand(cmd);
    let response;
    if (COMMANDS[parsedCmd[0]]) {
        if (parsedCmd.length > 1 && parsedCmd[1] === "-h") {
            response = COMMANDS.help.func([parsedCmd[0]]);
        } else {
            response = COMMANDS[parsedCmd[0]].func(
                parsedCmd.slice(1, parsedCmd.length)
            );
        }
    }

    // clear input value after running a command
    const input = document.getElementById("prompt-input");
    input.value = "";
}
// IIFE for setup
(() => {
    const lsLinks = readLinks();
    if (lsLinks) {
        links = lsLinks;
    }
    // Set Engine
    const savedEngine = readEngine();
    if (savedEngine) {
        searchUrl = savedEngine;
    }

    // Setup event listener for commands
    document.addEventListener("keydown", handleKeyPresses);

    const input = document.getElementById("prompt-input");
    input.value = "ls";
    return runCommand(input.value);
    // focus the prompt on init
    focusPrompt();

})();
