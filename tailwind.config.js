/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{html,rs}",
        "./index.html"
    ],
    theme: {
        extend: {
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
        },
    },
    darkMode: "class",
    plugins: [],
}

