/*
  Consts
*/

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
  "gruvbox-light",
  "nord",
  "dracula",
  "vice",
  "decaf",
];

const COMM = [
<<<<<<< HEAD
  "ls - list links",
  "add - add a site",
  "del - delete a site",
  "open - open a site",
  "books - list bookmarks",
  "search - search on google/ddg",
  "theme - change theme",
  "themes - list themes",
=======
  { name: 'ls', description: 'list links', args: 'none' },
  { name: 'add', description: 'add a site', args: 'name, url' },
  { name: 'del', description: 'delete a site', args: 'name' },
  { name: 'open', description: 'open a site', args: 'url' },
  { name: 'search', description: 'search on google/ddg', args: 'string' },
  { name: 'theme', description: 'change theme', args: 'theme' },
  { name: 'themes', description: 'list themes', args: 'none' },
  { name: 'clear', description: 'clear the "terminal"', args: 'none' }
>>>>>>> 6f2ae443113185de2a0ea1a625e651398bb64c14
];
