/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

module.exports = {
    content: {
        files: ["*.html","./src/**/*.{html,rs}"],
    },
    theme: {
        fontFamily: {
            sans: ['Inter', 'sans-serif'],
        },
        colors: {
            ...colors,
            'table-section': "#FFFFFF",
            'background': '#F9F9F8',
            'table-header-fill': '#EEEDF7',
            'table-row-fill': '#FFFFFF',
            'table-row-text-color': '#25213B',
            'table-header-text-color': '#25213B',
          },
        extend: {},
    },
    plugins: [],
    
}