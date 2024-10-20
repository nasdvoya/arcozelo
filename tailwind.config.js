/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/client/**/*.html", // Include all HTML files in client and subdirectories
        "./src/client/**/*.js",   // Include all JS files in client and subdirectories
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('daisyui'),
    ],
    daisyui: {
        themes: ["light", "dark", "cupcake"], // Add the themes you want to use
    },
}
