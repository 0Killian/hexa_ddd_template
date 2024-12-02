const { fontFamily } = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/interface/http/**/*.rs", "src/core/libs/ui/**/*.rs"],
  theme: {
    extend: {
      screens: {
        xs: "450px",
        md: "769px",
        md2: "896px",
        lg: "1025px",
        xl: "1441px",
      },
      fontFamily: {},
      colors: {
        ...colors,
      },
      backgroundImage: {
        wireframe_card: "url('/assets/wireframe_card_background.svg')",
      },
    },
  },
  plugins: [require("@tailwindcss/forms"), require("@tailwindcss/typography")],
};
