/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/presenter/**/*.html"],
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
    require("daisyui"), 
  ],
  daisyui: {
    themes: ["pastel"],
  },
  flowbite: {

  }
};
