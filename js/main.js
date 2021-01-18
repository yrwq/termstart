/**
 * Update body theme.
 */
const currentTheme = localStorage.getItem('theme')
if (currentTheme != null || undefined) document.body.classList.add(currentTheme)

/**
 * Variables.
 */
const searchEngine = localStorage.getItem('engine') || Engines.ddg

/**
 * Handle input.
 */
addEventListener("keydown", e => {
    if (e.code == 'Space' || 'Enter') document.getElementById('input').focus()
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
window.onload = () => document.getElementById('input').focus()
