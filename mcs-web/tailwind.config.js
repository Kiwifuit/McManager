/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{tsx,ts}"
  ],
  theme: {
    extend: {},
    colors: {
      // Dark Mode
      "dark-bg": "#202020",
      "dark-fg": "#eaeaea",
      "dark-accent": "#303030",

      // Light Mode
      "light-bg": "#ebebeb",
      "light-fg": "#1c1c1c",
      "light-accent": "#cccccc",

      // ServerListItem component
      // "light-server-description"
      "dark-server-description": "#a3a2a2",
      "dark-server-background": "#404040",

      // Server Online/Offline indicator
      "server-offline": "#d62f2f",
      "server-online": "#2ed154",
    }
  },
  plugins: [],
  // darkMode: "class"
}

