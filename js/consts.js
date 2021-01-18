const Commands = [
    {
        data: { name: 'ls', description: 'list links', args: '' },
        exec: () => ls()
    },
    {
        data: { name: 'add', description: 'add a site', args: 'name, url' },
        exec: (...args) => add(...args)
    },
    {
        data: { name: 'del', description: 'deletes added site', args: 'name' },
        exec: (...args) => del(...args)
    },
    {
        data: { name: 'open', description: 'open a link', args: 'url' },
        exec: (...args) => open(...args)
    },
    {
        data: { name: 'search', description: 'search for a term on ddg/google', args: '[-c ddg/google] string' },
        exec: (...args) => search(...args)
    },
    {
        data: { name: 'help', description: 'show available commands', args: '' },
        exec: (...args) => help(...args)
    },
    {
        data: { name: 'clear', description: 'clear the "terminal"', args: '' },
        exec: () => clear()
    },
    {
        data: { name: 'themes', description: 'list all themes', args: '' },
        exec: () => themes()
    },
    {
        data: { name: 'theme', description: 'change theme', args: 'theme' },
        exec: (...args) => theme(...args)
    }
]

const Engines = {
    google: 'https://google.com/search?q=',
    ddg: 'https://duckduckgo.com/?q=',
    bing: 'https://www.bing.com/search?q='
}

const Themes = [
    'gruvbox-dark',
    'gruvbox-light',
    'nord',
    'dracula',
    'vice',
    'decaf',
    'pastel'
]