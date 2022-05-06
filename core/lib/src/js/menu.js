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
                    console.log(href, linkUrl, match);
                    return match;
                });
            console.log(item);
            if (item) {
                item.classList.add("menu-item--selected");
            }
        });
});