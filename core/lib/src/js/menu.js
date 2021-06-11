window.addEventListener("load", () => {
    document.querySelectorAll(".menu")
        .forEach(menus => {
            let entries = [...menus.querySelectorAll("a[href]")].sort((a, b) => {
                let aLength = new URL(a.getAttribute("href"), document.baseURI).toString().length;
                let bLength = new URL(b.getAttribute("href"), document.baseURI).toString().length;
                return aLength - bLength;
            });

            entries.forEach(link => {
                    let linkUrl = new URL(link.getAttribute("href"), document.baseURI).toString();
                    entries.forEach(entry => {
                        entry.classList.remove("menu-item--selected");
                    });
                    if (window.location.href.includes(linkUrl)) {
                        link.classList.add("menu-item--selected");
                    }
                });
        });

});