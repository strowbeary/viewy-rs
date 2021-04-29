window.addEventListener("load", () => {
    document.querySelectorAll(".menu a[href]")
        .forEach(link => {
            if(link.getAttribute("href") === window.location.pathname) {
                link.classList.add("menu-item--selected")
            } else {
                link.classList.remove("menu-item--selected")
            }
        })
});