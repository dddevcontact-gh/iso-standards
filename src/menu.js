const width = parseInt(window.outerWidth);
const height = parseInt(window.outerHeight);
const menu = document.getElementById('menu');
const container = document.getElementById('mobilenav'); // need to hide stuff inside this for desktop mode
const def_terms = document.getElementById('term-definition');

let term = "";
let def = "";

console.log(`width ${width}`);
console.log(`height ${height}`);

switch (true) {
    case width >= 1200:
        console.log("Using desktop overlay");
        menu.style.display = "none";
        container.
        break;
    case width >= 768:
        console.log("Using tablet overlay");
        menu.style.display = "block";
        break;
    case width < 768:
        console.log("Using mobile overlay");
        menu.style.display = "block";
        break;
}

document.addEventListener('DOMContentLoaded', async () => {
    try {
        const response = await fetch("https://iso-standards-delta.vercel.app/api/random_terms");

        if (!response.ok) {
            throw new Error(`HTTP error: ${response.status}`);
        }

        const data = await response.json();

        term = data.term;
        def = data.definition;

        def_terms.textContent = `${term}`;
        console.log(randomNumber);
    } catch (err) {
        console.error("Failed to fetch term:", err);
    }
});

def_terms.addEventListener('mouseover', (event) => {
    def_terms.textContent = `${definition}`;
})