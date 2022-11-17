window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".menu")
        .forEach((menus) => {
            let item = [...menus.querySelectorAll("a[href]")]
                .sort((a, b) => {
                    let aLength = a.getAttribute("href").length;
                    let bLength = b.getAttribute("href").length;
                    return bLength - aLength;
                })
                .find((link) => {
                    let linkUrl = link.getAttribute("href");
                    let href = window.location.href;
                    let match =  href.toString().includes(linkUrl);
                    return match;
                });
            if (item) {
                item.classList.add("menu-item--selected");
            }
        });
});