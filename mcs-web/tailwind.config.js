/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{tsx,ts}"
  ],
  theme: {
    extend: {},
    colors: {
      "dark-bg": "#202020",
      "dark-fg": "#eaeaea",
      "dark-accent": "#303030",
      "light-bg": "#ebebeb",
      "light-fg": "#1c1c1c",
      "light-accent": "#cccccc"
    }
  },
  plugins: [],
  // darkMode: "class"
}

