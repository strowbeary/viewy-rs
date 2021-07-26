window.addEventListener("load", () => {
    document.querySelectorAll(".menu")
        .forEach(menus => {
            let entries = [...menus.querySelectorAll("a[href]")].sort((a, b) => {
                let aLength = new URL(a.getAttribute("href"), document.baseURI).toString().length;
                let bLength = new URL(b.getAttribute("href"), document.baseURI).toString().length;
                return bLength - aLength;
            });

            entries.find((link) => {
                let linkUrl = new URL(link.getAttribute("href")).toString();
                window.location.path.includes(linkUrl)
            })
                .classList.add("menu-item--selected");
        });

});