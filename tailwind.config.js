/** @type {import('tailwindcss').Config} */
const withMT = require("@material-tailwind/react/utils/withMT");

module.exports = withMT({
  content: ['./index.html', './src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        'dango-pink': '#ea005e',
        'dango-white': '#ffffff',
        'dango-green': '#16c60c',
        'plugin-github': '#161b22',
        'plugin-slack': '#4a154b',
        'plugin-trello': '#0075b9',
        'plugin-unknown': '#ea005e',
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
});
