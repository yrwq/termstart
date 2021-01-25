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
// Other mainstream browsers such as Vivaldi, Brave, etc. are Chrome(chromium) or Firefox based,
// Qutebrowser, Vimb, Suckless Surf identifies as safari however safari support tabs on Mac OS
// If i missed something please open an issue, or make a pull request, to add support for a browser.

if (os == "mac") {
  supported = ["Firefox", "Chrome", "Opera", "Safari", "Seamonkey"];
} else {
  supported = ["Firefox", "Chrome", "Opera", "Edge", "Chromium", "Seamonkey"];
}

/*
 * Focus the prompt...
*/

function focus_prompt() {
  document.getElementById("prompt-input").focus();
}

/*
 * Clear terminal's content
 */

function clear() {
  document.getElementById("links").innerHTML = "";
}

/*
 * Laziness kills me :(
*/

function fast_list() {
  return run_command("ls");
}

/*
 * Clear the prompt
 * Runs after every command
*/

function clear_prompt() {
  document.getElementById("prompt-input").value = "";
}

/*
 * List added links
*/

function list(input) {
  const cursor = get_links();

  return Object.entries(cursor).map(([key, value]) => {
    return {
      key, value
    };
  });
}

/*
 * List available themes
*/

function themes(input) {
  const cursor = get_themes();
  return Object.entries(cursor).map(([key, value]) => {
    return {
      key, value
    };
  });
}

function help(input) {
  if (input.length) {
    const final = input[input.length - 1];
    return COMM[final]
  } else {
    return COMM["help"]
  }
}

function commands(input) {
  return COMM["commands"]
}


/*
 * Open added link
*/

function open_link(input) {
  if (input.length) {

    let cursor = get_links();
    const final = input[input.length - 1];
    const target = cursor[final];

    if (supported.includes(result.parsedResult.browser.name)) {
      window.open(target, "_blank");
    } else {
      window.open(target, "_self");
    }
  }
}

/*
 * Add a link
*/

function add(input) {
  if (input.length == 2) {
    const path = input[0].split(" ");
    const url = format_url(input[1]);
    const parent = get_links();
    parent[path] = url;
    write_links();
  }
  fast_list();
}

/*
 * Delete an added link
*/

function del(input) {
  if (input.length) {

    const path = input[0].split(" ");
    const parent = get_links();

    delete parent[path];
    write_links();
  }
  fast_list();
}

/*
 * Search on the interweb.
*/

function search(input) {

  const search_string = input.join(' ');
  let target = search_url + search_string; // search_url is the default search engine

  if (supported.includes(result.parsedResult.browser.name)) {
    window.open(target, "_blank");
  } else {
    window.open(target, "_self");
  }
}

/*
 * Change theme
*/

function theme(input) {
  if (input.length) {
    document.body.className = "";
    document.body.classList.add(input[0]);
    write_theme(input[0]);
  }
}
