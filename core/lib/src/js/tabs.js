window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".tab-view")
        .forEach(tabView => {
            const tabs = tabView.querySelectorAll(".tab-view__tab-container__tab");

            async function desactivateAllTabs(except) {
                return tabs.forEach(tab => {
                    if (tab !== except) {
                        let tabId = tab.getAttribute("data-tabId");
                        detail.root.querySelector(`.tab-view__content-container__content[data-tabId="${tabId}"]`)
                            .style.display = "none";

                        tab.classList.remove("tab-view__tab-container__tab--active");

                    }
                })
            }

            tabs.forEach(tab => {
                tab.addEventListener("click", e => {
                    desactivateAllTabs(tab).then(() => {
                        let tabId = tab.getAttribute("data-tabId");
                        tab.classList.add("tab-view__tab-container__tab--active");
                        detail.root.querySelector(`.tab-view__content-container__content[data-tabId="${tabId}"]`)
                            .style.display = "block";
                    })
                });
            });

            tabView.querySelector(".tab-view__tab-container__tab").click();

        });
});