const LS_KEY = "links";
const LS_ENGINE_KEY = "engine";
const LS_THEME_KEY = "theme";
const types = {
    LINK: "link",
    DIR: "directory",
};
const ENGINES = {
    google: "https://google.com/search?q=",
    ddg: "https://duckduckgo.com/?q=",
    bing: "https://www.bing.com/search?q=",
};
const THEMES = [
    "gruvbox-dark",
    "nord",
];
// Get browser and os
var result = bowser.getParser(window.navigator.userAgent);
var userAgent = window.navigator.userAgent,
    platform = window.navigator.platform,
    macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
    windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'],
    iosPlatforms = ['iPhone', 'iPad', 'iPod'],
    os = null;

if (macosPlatforms.indexOf(platform) !== -1) {
    os = 'mac';
} else if (iosPlatforms.indexOf(platform) !== -1) {
    os = 'ios';
} else if (windowsPlatforms.indexOf(platform) !== -1) {
    os = 'windows';
} else if (/Android/.test(userAgent)) {
    os = 'android';
} else if (!os && /Linux/.test(platform)) {
    os = 'linux';
}

// Support Browser
// Mainstream browsers (Vivaldi, Brave, etc) are Chrome or Firefox based,
// Qutebrowser, Vimb, Suckless Surf identifies as safari however safari support tabs on Mac OS
// If i missed something please open an issue, or make a pull request, to add support for a browser.

if (os == 'mac') {
    var supported = ["Firefox", "Chrome", "Opera", "Safari", "Seamonkey"];
} else {
    var supported = ["Firefox", "Chrome", "Opera", "Edge", "Chromium", "Seamonkey"];
}
    
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
    }
}

function themeWriter() {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<ul class='ls-links'>";

    THEMES.forEach(add);

    function add(item){
	inner += '<li class="ls-item">' + item + '</li>'
    }
    
    inner = inner + "</ul>";
    outputNode.innerHTML = inner;
    document.getElementById("links").innerHTML = "";
    terminal.appendChild(outputNode);
}

function writer(output = "") {
    if (Array.isArray(output)) {
	listWriter(output);
    }
}

/*
 * Commands
*/

function focusPrompt() {
    document.getElementById("prompt-input").focus();
}

function fastList() {
    const input = document.getElementById("prompt-input");
    return runCommand("ls");
}

function clearPrompt() {
    const input = document.getElementById("prompt-input");
    input.value = "";
}

function list(input) {
    const cursor = getCurrentCursor();
    return Object.entries(cursor).map(([key, value]) => {
        return {
	    key,
	    type: locationType(value), // Determine if dir or link
        };
    });
}

function themes(input) {
    // TODO
}

// Open a link in a new tab
function openLink(input) {
    if (input.length) {
	const path = input[0].split("/");
	const target = locatePath(path);
	
	if (supported.includes(result.parsedResult.browser.name)) {
	    window.open(target, "_blank");
	} else {
	    window.open(target, "_self");
	}
    }
}

function touch(input) {
    if (input.length == 2) {
	const path = input[0].split("/");
	const url = formatUrl(input[1]);
	const parent = locateParentPath(path);
	const target = path[path.length - 1];
	parent[target] = url;
	writeLinks();
    }
    fastList();
}

function rm(input) {
    if (input.length) {
	const path = input[0].split("/");
	const parent = locateParentPath(path);
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
	const searchString = command[0];
	window.open(currentSearchUrl + searchString, "_blank");
    }
}

function theme(input) {
    if (input.length) {
	document.body.className = "";
	document.body.classList.add(input[0]);
	writeTheme(input[0]);
    }
}

/*
 * Cli
*/
// available commands
const COMMANDS = {
    // List links
    ls: {
        func: joinWriter(list, listWriter)
    },
    // Open a link
    open: {
        func: joinWriter(openLink, writer)
    },
    // Add a link
    add: {
        func: joinWriter(touch, writer)
    },
    // Delete a link
    del: {
        func: joinWriter(rm, writer)
    },
    // search with ddg or google
    search: {
        func: joinWriter(search, writer)
    },
    theme: {
	func: joinWriter(theme, writer),
    },
    themes: {
	func: joinWriter(themes, themeWriter)
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
    }
}

// user commands
function runCommand(cmd) {
    const parsedCmd = parseCommand(cmd);
    let response;
    response = COMMANDS[parsedCmd[0]].func(
        parsedCmd.slice(1, parsedCmd.length)
    );
    clearPrompt();
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
    // Get current theme
    const currentTheme = readTheme();
    theme([currentTheme]);
    
    document.addEventListener("keydown", handleKeyPresses);
    fastList();
})();
