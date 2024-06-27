/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/manager/**/*.html", "./templates/session/**/*.html"],
  theme: {
    fontFamily: {
      sans: ["Inconsolata", "sans-serif"],
    },
    extend: {
      fontFamily: {
        logo: "Kristi, cursive",
        label: "Pathway Gothic One, sans-serif",
      },
      colors: {
        primary: {
          50: "#eff6ff",
          100: "#dbeafe",
          200: "#bfdbfe",
          300: "#93c5fd",
          400: "#60a5fa",
          500: "#3b82f6",
          600: "#2563eb",
          700: "#1d4ed8",
          800: "#1e40af",
          900: "#1e3a8a",
          950: "#172554",
        },
      },
    },
  },
  plugins: [require("flowbite/plugin")],
  daisyui: {
    themes: ["pastel"],
  },
  flowbite: {},
};

// @config "./tailwindcss-config.js";
