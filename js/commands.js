/*
  Commands
*/

// Get browser and os
let result = bowser.getParser(window.navigator.userAgent);
let userAgent = window.navigator.userAgent,
  platform = window.navigator.platform,
  macosPlatforms = ["Macintosh", "MacIntel", "MacPPC", "Mac68K"],
  windowsPlatforms = ["Win32", "Win64", "Windows", "WinCE"],
  iosPlatforms = ["iPhone", "iPad", "iPod"],
  os = null;

if (macosPlatforms.indexOf(platform) !== -1) {
  os = "mac";
} else if (iosPlatforms.indexOf(platform) !== -1) {
  os = "ios";
} else if (windowsPlatforms.indexOf(platform) !== -1) {
  os = "windows";
} else if (/Android/.test(userAgent)) {
  os = "android";
} else if (!os && /Linux/.test(platform)) {
  os = "linux";
}

// Supported Browsers
// This needs to be done because not every browser can open a link in a new tab, example: surf, vimb
// Other mainstream browsers such as Vivaldi, Brave, etc. are Chrome or Firefox based,
// Qutebrowser, Vimb, Suckless Surf identifies as safari however safari support tabs on Mac OS
// If i missed something please open an issue, or make a pull request, to add support for a browser.

if (os == "mac") {
  supported = ["Firefox", "Chrome", "Opera", "Safari", "Seamonkey"];
} else {
  supported = ["Firefox", "Chrome", "Opera", "Edge", "Chromium", "Seamonkey"];
}

function focusPrompt() {
  document.getElementById("prompt-input").focus();
}

function fastList() {
  return runCommand("ls");
}

function clearPrompt() {
  document.getElementById("prompt-input").value = "";
}

function list(input) {
  const cursor = get_cursor(position);
  return Object.entries(cursor).map(([key, value]) => {
    return {
      key, value
    };
  });
}

function themes(input) {
  // TODO
}

function command(input) {
  // TODO
}

// Open a link in a new tab
function open_link(input) {
  if (input.length) {

    const target = locate_path(input);

    if (supported.includes(result.parsedResult.browser.name)) {
      window.open(target, "_blank");
    } else {
      window.open(target, "_self");
    }
  }
}

function add(input) {
  if (input.length == 2) {
      const path = input[0].split(" ");
      const url = formatUrl(input[1]);
      const parent = locate_parent_path(path);
      const target = path[path.length - 1];
      parent[target] = url;
      writeLinks();
  }
  fastList();
}

function del(input) {
  if (input.length) {
    const path = input[0].split(" ");
    const parent = locate_parent_path(path);
    const target = path[path.length - 1];
    delete parent[target];
    writeLinks();
  }
  fastList();
}

function search(input) {
  const { command, flags } = extractFlags(input, {
    c: "string",
  });
  let currentSearchUrl = searchUrl;
  if (flags.c) {
    currentSearchUrl = ENGINES[flags.c] ? ENGINES[flags.c] : flags.c;
    if (!command[0]) {
      // Set saved engine to given
      searchUrl = currentSearchUrl;
      writeEngine(currentSearchUrl);
    }
  }
  if (command && command[0]) {
    const searchString = command.join(' ');
    if (supported.includes(result.parsedResult.browser.name)) {
      window.open(target, "_blank");
    } else {
      window.open(target, "_self");
    }
  }
}

function theme(input) {
  if (input.length) {
    document.body.className = "";
    document.body.classList.add(input[0]);
    writeTheme(input[0]);
  }
}
