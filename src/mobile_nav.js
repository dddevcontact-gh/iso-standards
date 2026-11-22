const menu = document.getElementById("menu");
const sidenav = document.getElementById("sidenav");

menu.addEventListener("click", () => {
    sidenav.classList.toggle("open");
});