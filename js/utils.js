const linksDiv = document.getElementById('links')

const resetLinksDiv = () => {
    for (let i = 0; i < linksDiv.children.length; i++) {
        linksDiv.children[i].remove()
    }
}

const writeList = (elements = []) => {
    resetLinksDiv()

    const parent = document.createElement('ul')
    parent.classList.add('padding')

    elements.forEach(element => {
        const list = document.createElement('li')
        const child = document.createElement(element.type || 'p')
        const arrowSpan = document.createElement('span')
        arrowSpan.classList.add('material-icons', 'md-36')
        arrowSpan.innerText = 'arrow_right_alt'
        child.appendChild(arrowSpan)
        child.classList.add('item')
        child.setAttribute('href', element.href || '')
        child.append(element.text)
        list.appendChild(child)
        parent.appendChild(list)
    })

    linksDiv.appendChild(parent)
}

const write = (string) => {
    resetLinksDiv()

    const child = document.createElement('p')
    child.classList.add('title', 'padding')
    child.innerText = string

    linksDiv.appendChild(child)
}

const getLinks = () => JSON.parse(localStorage.getItem('links') || "[]")

const addLink = (name = 'default', url = '') => {
    const links = getLinks()
    links.push({ name: name, url: url })
    localStorage.setItem('links', JSON.stringify(links || []))
}

const delLink = (name = 'default') => {
    const links = getLinks()
    const link = links.find(link => link.name == name)
    const index = links.indexOf(link)
    delete links[index]

    const newLinks = []
    links.forEach(link => link ? newLinks.push(link) : null)

    links[0] != undefined 
        ? localStorage.setItem('links', JSON.stringify(newLinks)) 
        : localStorage.removeItem('links')
}

const updateClock = () => {
    const date = new Date()
    const hour = date.getHours()
    const minutes = date.getMinutes()
    document.getElementById('clock').innerHTML = `${hour < 10 ? `0${hour}` : hour}:${minutes < 10 ? `0${minutes}` : minutes}`
}