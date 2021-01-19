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

function get_links() {
  let pos = [];
  let cursor = links;
  pos.forEach((p) => {
    cursor = cursor[p];
  });
  return cursor;
}

function get_themes() {
  let pos = [];
  let cursor = THEMES;
  pos.forEach((p) => {
    cursor = cursor[p];
  });
  return cursor;
}

function get_commands() {
  let pos = [];
  let cursor = COMM;
  pos => {
    cursor = cursor[pos];
  };
  return cursor;
}

function push_command(cmd) {
  const prompt = document.getElementById("prompt-input");
  prompt.value = cmd;
  focus_prompt();
}

// Parse command input by keeping strings in "" together as an single item
function parse_command(input) {
  const re = /"([^"]+)"|([^\s]+)/g;
  const parsed_cmd = [];
  let temp;
  while ((temp = re.exec(input)) !== null) {
    const val = temp[1] || temp[2]; // Get the correct capture group
    parsed_cmd.push(val);
  }
  return parsed_cmd;
}

function format_url(url) {
  let finalUrl = url;
  if (!/^http|https:\/\//.test(finalUrl)) {
    finalUrl = "https://" + finalUrl;
  }
  return finalUrl;
}

/*
 * Cursors
*/

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
