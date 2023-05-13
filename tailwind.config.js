/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html.tera",
  "./static/**/*.js",
  ],
  theme: {
    extend: {
      fontFamily: {
        glober: ["Glober","sans-serif"],
    },},
  },
  plugins: [
    require('tailwindcss'),
    require('autoprefixer'),
  ],
}

