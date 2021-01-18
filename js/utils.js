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