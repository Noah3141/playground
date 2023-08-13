/** @type {import('tailwindcss').Config} */
/* https://tailwindcss.com/docs/installation */

module.exports = {
    content: ["./src/**/*.{html,rs}", "./index.html"], // Set to look in these HTML and Rust files for tailwind phrases to unpack
    theme: {
        extend: {},
    },
    plugins: [],
};
