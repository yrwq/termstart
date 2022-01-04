/*
  Consts
*/

/*
 * Local storage items
*/

const LS_KEY = "links";
const LS_ENGINE_KEY = "engine";
const LS_THEME_KEY = "theme";

/*
 * All available search engines should be added here
 * TODO Add more search engines
*/

const ENGINES = {
  google: "https://google.com/search?q=",
  ddg: "https://www.duckduckgo.com/?q=",
  bing: "https://www.bing.com/search?q=",
  yahoo: "https://www.search.yahoo.com/search?p=",
};

/*
 * All available themes should be added here
*/

const THEMES = [
  "material",
  "gruvbox-dark",
  "gruvbox-light",
  "nord",
  "solarized",
  "tomorrow",
  "dracula",
  "vice",
  "decaf",
  "pastel",
];

/*
 * All available commands should be added here,
 * These will be shown in the help and error message
*/

const COMM_ls = {
  name: 'ls',
  description: 'list links',
  usage: 'ls',
  longdesc: 'Lists all added "bookmarks"<br>you can open open added links if you<br> click them, or with open "name"'
}

const COMM_add = {
  name: 'add',
  description: 'add a site',
  usage: 'add "name" "url"',
  longdesc: 'Add a site to your "bookmarks"<br>after adding a site you can <br>list, open or delete them'
}

const COMM_del = {
  name: 'del',
  description: 'delete a site',
  usage: 'del "name"',
  longdesc: 'Deletes an added "bookmark"'
}

const COMM_open = {
  name: 'open',
  description: 'open a site',
  usage: 'open "name"',
  longdesc: 'Open an added "bookmark" in a new tab.'
}

const COMM_search = {
  name: 'search',
  description: 'search on the interweb',
  usage: 'search "interesting topic"',
  longdesc: `Search for keywords or topics on the interweb`,
}

const COMM_engine = {
  name: 'engine',
  description: 'change search engine',
  usage: 'engine "engine-name"',
  longdesc: 'Change the default search engine of termstart, <br>you can list all available <br>search engines with the command engines'
}

const COMM_engines = {
  name: 'engines',
  description: 'list search engines',
  usage: 'engines',
  longdesc: 'Lists all available search engines, <br>you can apply one of them with<br> the command engine "engine-name"'
}

const COMM_theme = {
  name: 'theme',
  description: 'change theme',
  usage: 'theme "theme-name"',
  longdesc: 'Change the colors of termstart, <br>you can list all available <br>themes with the command themes'
}

const COMM_themes = {
  name: 'themes',
  description: 'list themes',
  usage: 'themes',
  longdesc: 'Lists all available themes, <br>you can apply one of them with<br> the command theme "theme-name"'
}

const COMM_clear = {
  name: 'clear',
  description: 'clear the "terminal"',
  usage: 'clear',
  longdesc: 'Literally clears the "terminal"'
}

const COMM_help = {
  name: 'help',
  description: 'Literally helps you',
  usage: 'help "command"',
  longdesc: 'To get a list of all available<br> commands, type: "commands"'
}

const COMM_commands = [ "help", "clear", "add", "del", "ls", "open", "search", "engine", "engines", "theme", "themes"]

const COMM = {
  ls: COMM_ls,
  add: COMM_add,
  del: COMM_del,
  open: COMM_open,
  search: COMM_search,
  engine: COMM_engine,
  engines: COMM_engines,
  theme: COMM_theme,
  themes: COMM_themes,
  clear: COMM_clear,
  help: COMM_help,
  commands: COMM_commands,
}

