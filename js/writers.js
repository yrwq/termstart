/*
  Writers
*/

function list_writer(output) {
  if (Array.isArray(output)) {
    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<table class='ls-links'>";

    half1 = output.slice(output.length / 2);
    half2 = output.slice(0, output.length / 2);

    if (output.length <= 6) {
      inner =
        inner +
        output
          .map(
            (item) =>
            `<th class="ls-item"><a target='_blank' href='${links[item.key]}'>${item.key}</a></th>`
          )
          .join("");
    } else {
      inner =
        inner +
        half1
          .map(
            (item) =>
            `<th class="ls-item"><a target='_blank' href='${links[item.key]}'>${item.key}</a></th>`
          )
          .join("");
      inner = inner + "<tr></tr>";
      inner =
        inner +
        half2
          .map(
            (item) =>
            `<th class="ls-item"><a target='_blank' href='${links[item.key]}'>${item.key}</a></th>`
          )
          .join(" ");
    }

    inner = inner + "</table>";
    outputNode.innerHTML = inner;

    run_command("clear");
    terminal.appendChild(outputNode);

  }
}


function theme_writer(item) {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("ls");
  let inner = "<list class='ls-links'>";

  inner =
    inner +
    item
      .map(
        (item) =>
          `<li class="ls-item"> <a href="javascript:run_command('theme ${item.value}');">
        ${item.value}</a></li>`
      )
      .join("");

  inner = inner + "</list>";
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

function command_writer(command) {
  const terminal = document.getElementById("links");
  const outputNode = document.createElement("div");
  outputNode.classList.add("ls");
  let inner = "<div class='ls-links' align='center' style='margin: 20px;'>";

  inner += `<p class="ls-item">${command} </p>`

  inner = inner + "</div>";
  outputNode.innerHTML = inner;
  document.getElementById("links").innerHTML = "";
  terminal.appendChild(outputNode);
}
