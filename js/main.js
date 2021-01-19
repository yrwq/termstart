/**
 * Variables.
 */
const currentTheme = localStorage.getItem('theme')
const searchEngine = localStorage.getItem('engine') || Engines.ddg
const input = document.getElementById('input')
const last = document.getElementById('last')

const result = bowser.getParser(window.navigator.userAgent);
const userAgent = window.navigator.userAgent,
    platform = window.navigator.platform,
    macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
    windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'],
    iosPlatforms = ['iPhone', 'iPad', 'iPod'],
    os = null;


/**
 * Supported Browsers
 */
if (macosPlatforms.indexOf(platform) !== -1) os = 'ac'
else if (iosPlatforms.indexOf(platform) !== -1) os = 'ios'
else if (windowsPlatforms.indexOf(platform) !== -1) os = 'windows'
else if (/Android/.test(userAgent)) os = 'android'
else if (!os && /Linux/.test(platform)) os = 'linux'
if (os == 'mac') supported = ['Firefox', 'Chrome', 'Opera', 'Safari', 'Seamonkey']
else supported = ['Firefox', 'Chrome', 'Opera', 'Edge', 'Chromium', 'Seamonkey']

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