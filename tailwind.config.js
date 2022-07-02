/** @type {import('tailwindcss').Config} */
module.exports = {
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
        'plugin-unknown': 'purple-400',
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
