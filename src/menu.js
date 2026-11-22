const container = document.getElementById('mobilenav'); // need to hide stuff inside this for desktop mode
const def_terms = document.getElementById('term-definition');

let term = "";
let def = "";

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
    } catch (err) {
        console.error("Failed to fetch term:", err);
    }
});

def_terms.addEventListener('mouseover', (event) => {
    def_terms.textContent = `${def}`;
})

def_terms.addEventListener('mouseout', (event) => {
    def_terms.textContent = `${term}`;
})