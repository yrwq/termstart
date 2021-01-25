/*
 * Main
*/

/*
 * Available commands
*/

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
    func: join_writer(help, help_writer),
  },
  // clear
  clear: {
    func: join_writer(clear, writer),
  },
  commands: {
    func: join_writer(commands, command_writer),
  },
};

let search_url = ENGINES.ddg; // default search engine
let links = {};

function handle_key_presses(e) {
  let prompt = document.getElementById("prompt");

  if (e.keyCode === 13) {
    const input = document.getElementById("prompt-input");
    return run_command(input.value);
  }

}


function run_command(cmd) {
  const parsed_cmd = parse_command(cmd);
  let response;
  let prompt = document.getElementById("prompt");

  try {
    response = COMMANDS[parsed_cmd[0]].func(
      parsed_cmd.slice(1, parsed_cmd.length)
    );
  }
  catch {
    // error_writer();
  }

  clear_prompt();

  prompt.innerHTML =
    `<span class="purple material-icons md-36">chevron_right</span>
    ${parsed_cmd[0]}
    <span id=clock></span>`;
}

(() => {

  const ls_links = read_links();
  if (ls_links) {
    links = ls_links;
  }

  const saved_engine = read_engine();
  if (saved_engine) {
    search_url = saved_engine;
  }

  const current_theme = read_theme();
  theme([current_theme]);

  document.addEventListener("keydown", handle_key_presses);
  document.body.addEventListener('keypress', focus_prompt);

  fast_list();

})();
