/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      transitionProperty: {
        "max-height": "max-height",
      },
    },
  },
  plugins: [require("daisyui")],
};
