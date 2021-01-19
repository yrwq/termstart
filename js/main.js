/**
 * Variables.
 */
const currentTheme = localStorage.getItem('theme')
const searchEngine = localStorage.getItem('engine') || Engines.ddg
const input = document.getElementById('input')
const last = document.getElementById('last')

/**
 * Update body theme.
 */
if (currentTheme != null || undefined) document.body.classList.add(currentTheme)

/**
 * Handle input.
 */
addEventListener('keydown', e => {
    if (e.code == 'Space' || 'Enter') input.focus()
    if (e.code == 'Enter' && input.value.length > 0) {
        const string = input.value
        const parsed = string.split(' ')
        const name = parsed[0]
        const cmd = Commands.find(cmd => cmd.data.name == name)
        const args = string.replace(name, '').split(' ')
        args.splice(0, 1)

        /**
         * @TODO add "" support.
         */
        if (cmd) cmd.exec(...args)

        input.value = ''
        last.innerText = name
    }
})

/**
 * Update clock.
 */
setInterval(() => {
    updateClock()
}, 100);

/**
 * Onload
 */
window.onload = () => {
    input.value = ''
    last.innerText = 'ls'
    ls()
    input.focus()
}