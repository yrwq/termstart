/*
  Commands
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

function command(input) {
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
