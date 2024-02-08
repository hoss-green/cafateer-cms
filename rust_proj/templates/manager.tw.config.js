/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/manager/**/*.html"],
  theme: {
    fontFamily: {
      sans: ["Inconsolata", "sans-serif"],
    },
    extend: {
      fontFamily: {
        logo: "Kristi, cursive",
        label: "Pathway Gothic One, sans-serif",
      },
    },
  },
  plugins: [
    require("flowbite/plugin"),
  ],
  daisyui: {
    themes: ["pastel"],
  },
  flowbite: {

  }
};

// @config "./tailwindcss-config.js";
