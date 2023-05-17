window.addEventListener("startViewy", ({detail}) => {

    window.addEventListener("popstate", (event) => {
        let itemId = event.target.location.hash;
        detail.root.querySelectorAll(".table-of-contents__item")
            .forEach((item) => {
                console.log(item.getAttribute("href"), itemId);
                if (item.getAttribute("href") === itemId) {
                    item.classList.add("active");
                } else {
                    item.classList.remove("active");
                }
            });
    });

    detail.root.querySelectorAll(".table-of-contents")
        .forEach((toc) => {
            let items = toc.querySelectorAll(".table-of-contents__item");

            let observer = new IntersectionObserver((entries, observer) => {
                entries.forEach((entry) => {

                    if (entry.intersectionRatio >= 0.3) {
                        let id = entry.target.getAttribute("id");

                        items.forEach((item) => {

                                if (item.getAttribute("href") === `#${id}`) {
                                    item.classList.add("active");
                                } else {
                                    item.classList.remove("active");
                                }
                            });
                    }
                });
            }, {
                root: document.getElementById(toc.getAttribute("data-root")),
                rootMargin: "0px",
                threshold: 0.3,
            });
            [...items]
                .map((item) => {
                    return document.getElementById(item.getAttribute("href").replace("#", ""));
                })
                .forEach((target) => {
                    console.log(target);
                    if (target) {
                        observer.observe(target);
                    }
                })
        });
});