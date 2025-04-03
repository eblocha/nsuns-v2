/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}", "index.html"],
  theme: {
    extend: {
      fontSize: {
        '10xl': ['10rem', '10rem']
      },
    },
  },
  plugins: [],
};
