/*
  Utils
*/

function safe_parse(input) {
  try {
    return JSON.parse(input) || {};
  } catch {
    return {};
  }
}

function join_writer(command, writer) {
  return (input) => {
    writer(command(input));
  };
}

function get_cursor(pos) {
  let cursor = links;
  pos.forEach((p) => {
    cursor = cursor[p];
  });
  return cursor;
}

function get_current_cursor() {
  return get_cursor(position);
}

function push_command(cmd) {
  const prompt = document.getElementById("prompt-input");
  prompt.value = cmd;
  focus_prompt();
}

function locate_path(path) {
  let cursor = locate_parent_path(path);
  if (path.length) {
    const final = path[path.length - 1];
    return cursor[final];
  }
  return cursor;
}

function locate_parent_path(fullPath) {
  const path = fullPath.slice(0, fullPath.length - 1);
  let cursor = get_current_cursor();
  return cursor;
}

// Parse command input by keeping strings in "" together as an single item
function parse_command(input) {
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
function extract_flags(command, flagMap = {}) {
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

function format_url(url) {
  let finalUrl = url;
  if (!/^http|https:\/\//.test(finalUrl)) {
    finalUrl = "https://" + finalUrl;
  }
  return finalUrl;
}

// LocalStorage Interaction Functions
function read_links() {
  return safe_parse(localStorage.getItem(LS_KEY));
}

function write_links() {
  localStorage.setItem(LS_KEY, JSON.stringify(links));
}

function read_engine() {
  return localStorage.getItem(LS_ENGINE_KEY);
}

function write_engine(url) {
  localStorage.setItem(LS_ENGINE_KEY, url);
}

function read_theme() {
  return localStorage.getItem(LS_THEME_KEY);
}

function write_theme(theme) {
  localStorage.setItem(LS_THEME_KEY, theme);
}
