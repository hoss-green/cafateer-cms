/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
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
    require("daisyui"), 
  ],
  daisyui: {
    themes: ["pastel"],
  },
  flowbite: {

  }
};
