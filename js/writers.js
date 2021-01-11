/*
  Writers
*/

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
	inner += '<li class="ls-item">' + item + '</li>';
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

function errorWriter() {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<ul class='ls-links'>";
    
    inner += '<h3> <p> Available commands </p></h3>';
    COMM.forEach(add);
    
    function add(item){
	inner += '<li class="ls-item">' + item + '</li>';
    }
    
    inner = inner + "</ul>";
    outputNode.innerHTML = inner;
    document.getElementById("links").innerHTML = "";
    terminal.appendChild(outputNode);
}

function clearWriter() {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<ul class='ls-links'>";
    
    inner = inner + "</ul>";
    outputNode.innerHTML = inner;
    document.getElementById("links").innerHTML = "";
    terminal.appendChild(outputNode);
}
