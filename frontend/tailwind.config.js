/** @type {import('tailwindcss').Config} */
module.exports = {
  content:[
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        deepblue: {
          500: '#023047',
        },
        amber: {
          500: '#fd8500',
        },
        alabaster: {
          500: '#e0e1dd',
        }
      }
    },
  },
  plugins: [],
}