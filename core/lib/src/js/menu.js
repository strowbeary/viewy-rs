window.addEventListener("load", () => {
    document.querySelectorAll(".menu a[href]")
        .forEach(link => {
            if(link.getAttribute("href") === window.location.pathname) {
                link.classList.add("current")
            } else {
                link.classList.remove("current")
            }
        })
});