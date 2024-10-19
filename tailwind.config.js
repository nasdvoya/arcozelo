/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/client/*.{html,js}"],
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

