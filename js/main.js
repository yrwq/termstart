/**
 * Update body theme.
 */
const currentTheme = localStorage.getItem('theme') || 'gruvbox-dark'
document.body.classList.add(currentTheme)

/**
 * Handle input.
 */

const searchEngine = localStorage.getItem('engine') || Engines.ddg