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
        .map(
          (item) =>
            `<li class="ls-item"><span class="material-icons md-36">arrow_right_alt</span><a target='_blank' href='${links[item.key]}'>${
              item.key
            }</a></li>`
        )
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

  function add(item) {
    inner += `<li class="ls-item"><span class="material-icons md-36">arrow_right_alt</span>${item}</li>`;
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

function errorWriter(error_cmd) {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("ls");
  let inner = "<ul class='ls-links'>";

  inner += "<h3 class='purple'>Available commands</h3>";
  COMM.forEach(add);

  function add(item) {
    inner += `<li class="ls-item"><span class="material-icons md-36">arrow_right_alt</span>${item.name} - ${item.description}</li>`;
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
