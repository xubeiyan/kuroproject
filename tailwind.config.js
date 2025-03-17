/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "1st": "#dff2eb",
        "2nd": "#b9e5e8",
        "3rd": "#7ab2d3",
        "3rd-light": "#4bb6ed",
        "4th": "#4a628a",
        alert: "#d14028",
        "alert-light": "#de2700",
        "2nd-text": "#6e6e6e",
      },
    },
  },
  plugins: [],
};
