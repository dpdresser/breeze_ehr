/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./*.html",
    "./src/**/*.{js,ts,jsx,tsx,html}"
  ],
  theme: {
    extend: {
      colors: {
        "seafoam": {
          light: "#a8f9ff",
          DEFAULT: "#9ae5e6",
          dark: "#81a094"
        },
        "terra": {
          light: "#775b59",
          dark: "#32161f"
        }
      },
    },
  },
  plugins: [],
}
