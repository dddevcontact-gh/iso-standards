const width = parseInt(window.outerWidth);
const height = parseInt(window.outerHeight);
const menu = document.getElementById('menu');
const container = document.getElementByIdName('mobilenav'); // need to hide stuff inside this for desktop mode

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