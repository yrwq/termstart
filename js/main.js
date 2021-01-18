const COMMANDS = {
  // List links
  ls: {
    func: join_writer(list, list_writer),
  },
  // Open a link
  open: {
    func: join_writer(open_link, writer),
  },
  // Add a link
  add: {
    func: join_writer(add, writer),
  },
  // Delete a link
  del: {
    func: join_writer(del, writer),
  },
  // search with ddg or google
  search: {
    func: join_writer(search, writer),
  },
  // change theme
  theme: {
    func: join_writer(theme, writer),
  },
  // list themes
  themes: {
    func: join_writer(themes, theme_writer),
  },
  // help
  help: {
    func: join_writer(command, error_writer),
  },
  // clear
  clear: {
    func: join_writer(command, clear_writer),
  },
};

let searchUrl = ENGINES.ddg; // default search engine
let links = {};
let position = []; // Determines where in the link tree we are currently

function handle_key_presses(e) {
  if (e.keyCode === 13) {
    // Enter
    const input = document.getElementById("prompt-input");
    return run_command(input.value);
  }
}

function run_command(cmd) {
  const parsedCmd = parse_command(cmd);
  let response;
  let prompt = document.getElementById("prompt");

  try {
    response = COMMANDS[parsedCmd[0]].func(
      parsedCmd.slice(1, parsedCmd.length)
    );
  }

  // Handling errors

  catch (err) {

    const terminal = document.getElementById("links");
    const outputNode = document.createElement("div");
    outputNode.classList.add("ls");
    let inner = "<ul class='ls-links'>";

    inner += `<h3 class='purple'> Unknown command: ${parsedCmd[0]}</h3>`;
    COMM.forEach(add);

    function add(item) {
      inner += `<li class="ls-item"><span class="material-icons md-36">arrow_right_alt</span>${item.name} - ${item.description}</li>`;
    }

    inner = inner + "</ul>";
    outputNode.innerHTML = inner;
    document.getElementById("links").innerHTML = "";
    terminal.appendChild(outputNode);

  }

  clear_prompt();

  prompt.innerHTML =
    `<span class="purple material-icons md-36">chevron_right</span>
    ${parsedCmd[0]}
    <span id=clock></span>`;
}

(() => {

  const lsLinks = read_links();
  if (lsLinks) {
    links = lsLinks;
  }

  const savedEngine = read_engine();
  if (savedEngine) {
    searchUrl = savedEngine;
  }

  const currentTheme = read_theme();
  theme([currentTheme]);

  document.addEventListener("keydown", handle_key_presses);
  fast_list();

})();
