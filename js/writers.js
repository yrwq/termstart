/*
  Writers
*/

function list_writer(output) {
  if (Array.isArray(output)) {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<div class='ls-links'>";

    inner =
      inner +
      output
        .map(
          (item) =>
            `<p class="ls-item"><a target='_blank' href='${links[item.key]}'>${item.key}</p>`
        )
        .join("");


    inner = inner + "</div>";
    outputNode.innerHTML = inner;

    run_command("clear");
    terminal.appendChild(outputNode);
  }
}

function theme_writer(item) {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("ls");
  let inner = "<div class='ls-links'>";

  inner =
    inner +
    item
      .map(
        (item) =>
          `<p class="ls-item"><span class="material-icons md-36">arrow_right_alt</span>
          <a href="javascript:run_command('theme ${item.value}');">
        ${item.value}</a></p>`
      )
      .join("");

  inner = inner + "</ul>";
  outputNode.innerHTML = inner;
  run_command("clear");
  terminal.appendChild(outputNode);
}

function writer(output = "") {
  if (Array.isArray(output)) {
    list_writer(output);
  }
}

function help_writer(command) {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("ls");
  let inner = "<div class='ls-links' align='center' style='margin: 20px;'>";

  inner += `<br> <p class="ls-item">${command.name} - ${command.description}</p> <br>`

  inner += `<p class="ls-item">Usage: ${command.usage}</p><br>`

  inner += `<p class="ls-item" >${command.longdesc}</li><br>`

  inner = inner + "</div>";
  outputNode.innerHTML = inner;
  document.getElementById("links").innerHTML = "";
  terminal.appendChild(outputNode);
}
