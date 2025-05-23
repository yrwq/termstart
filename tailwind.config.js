/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{html,rs}",
        "./index.html"
    ],
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                'github-dark': {
                    bg: '#0d1117',
                    text: '#c9d1d9',
                    border: '#30363d',
                    button: '#21262d',
                    'button-hover': '#30363d'
                },
                'github-light': {
                    bg: '#ffffff',
                    text: '#24292f',
                    border: '#d0d7de',
                    button: '#f6f8fa',
                    'button-hover': '#f3f4f6'
                }
            },
            animation: {
                "fadein": "fadein 0.1s linear",
                "handwave": "handwave 2.5s linear infinite",
            },
            keyframes: {
                "fadein": {
                     "0%": {
                        opacity: 0,
                    },
                    "100%": {
                        opacity: 1,
                    }
                },
            },
            transitionProperty: {
                'colors': 'color, background-color, border-color, text-decoration-color, fill, stroke',
            },
            transitionDuration: {
                'default': '200ms',
            },
            transitionTimingFunction: {
                'default': 'ease-in-out',
            },
        },
    },
    plugins: [
        require('tailwind-scrollbar')({ nocompatible: true }),
    ]
}

