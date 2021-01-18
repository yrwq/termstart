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
        const parsed = input.value.split(' ')
        const name = parsed[0]
        const cmd = Commands.find(cmd => cmd.data.name == name)
        const argsTemp = input.value.replace(`${name}`, '').split(' ')
        const args = argsTemp.join(' ').split(' ')
        console.log(args)
        /**
         * @TODO add switch with commands.
         */
        //if (cmd && cmd.data.args == '' ? true : args[1] ? true : false) cmd.exec(args)
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
window.onload = () => input.focus()
