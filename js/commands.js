const ls = () => {
    const links = getLinks()
    let temp = []

    links.forEach(link => temp.push({ type: 'a', href: link.href, text: link.name }))

    writeList(temp)
}

const add = (name, url) => {
    addLink(name, url)
    clear()
}

const del = (name) => {
    delLink(name)
    clear()
}

const open = (url) => {
    if (!url) return write('Invalid arguments.')
    window.open(url)
}

const search = (string) => {
    if (!string) return write('Invalid arguments.')
    const parsedArgs = string.split(' ')
    console.log(parsedArgs)
    if (parsedArgs[0] == '-c' && Object.keys(Engines).includes(parsedArgs[1])) {
        localStorage.setItem('engine', parsedArgs[1])
        location.reload()
    } else {
        window.open(`${Engines[searchEngine]}${string}`)
    }
}

const help = (name) => {
    let temp = []

    Commands.forEach(cmd => temp.push({ text: `${cmd.name} - ${cmd.description}` }))

    writeList(temp)
}

const clear = () => {
    resetLinksDiv()
    ls()
}

const themes = () => {
    let temp = []

    Themes.forEach(theme => temp.push({ text: theme }))

    writeList(temp)
}

const theme = (theme) => {
    if (!Themes.includes(theme)) localStorage.setItem('theme', 'gruvbox-dark')
    else localStorage.setItem('theme', theme)
    location.reload()
}