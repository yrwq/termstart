const ls = () => {
    const links = getLinks() || []
    let temp = []
    links.forEach(link => temp.push({ type: 'a', href: !link.url.startsWith('https://') ? `https://${link.url}` : link.url, text: link.name }))

    writeList(temp)
}

const add = (...args) => {
    if (args[0] && args[1]) addLink(args.join(' ').replace(args[0], '').replace(' ', ''), args[0])
    else return write('Invalid arguments.')
    clear()
}

const del = (...args) => {
    if (args[0]) delLink(args.join(' '))
    else return write('Invalid arguments.')
    clear()
}

const open = (...args) => {
    if (!args[0]) return write('Invalid arguments.')
    window.open(args[0])
}

const search = (...args) => {
    if (!args[0]) return write('Invalid arguments.')
    if (args[0] == '-c' && Object.keys(Engines).includes(args[1])) {
        localStorage.setItem('engine', args[1])
        location.reload()
    } else {
        if (supported.includes(result.parsedResult.browser.name)) {
            window.open(`${Engines[searchEngine]}${args.join(' ')}`, '_blank')
        } else {
            window.open(`${Engines[searchEngine]}${args.join(' ')}`, '_self')
        }
    }
}

const help = (...args) => {
    let temp = []

    Commands.forEach(cmd => temp.push({ text: `${cmd.data.name} - ${cmd.data.description}` }))

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

const theme = (...args) => {
    if (!Themes.includes(args[0])) localStorage.setItem('theme', 'gruvbox-dark')
    else localStorage.setItem('theme', args[0])
    location.reload()
}