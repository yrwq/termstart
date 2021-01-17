/*
  Utils
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

function readTheme() {
  return localStorage.getItem(LS_THEME_KEY);
}

function writeTheme(theme) {
  localStorage.setItem(LS_THEME_KEY, theme);
}
