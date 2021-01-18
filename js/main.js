const currentTheme = localStorage.getItem('theme') || 'gruvbox-dark'

document.body.classList.add(currentTheme)