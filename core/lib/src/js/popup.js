function closeAllPopover() {
    document.querySelectorAll(".popover").forEach(popover => {
        popover.removeAttribute("data-show");
    });
}

    window.addEventListener("startViewy", ({detail}) => {
        const popups = detail.root.querySelectorAll(".popup");

        function closeAll(excludedPopupIds) {
            //closeAllPopover();
            popups.forEach(popup => {
                let popupId = popup.getAttribute("data-attach-to");

                if (!excludedPopupIds.includes(popupId)) {
                    popup.classList.remove("visible");
                }
            });
        }

        function open(popupId) {
            let popup = document.querySelector(`.popup[data-attach-to="${popupId}"]`);
            let opennedPopups = [...detail.root.querySelectorAll(`.popup.visible`)]
                .map(popup => popup.getAttribute("data-attach-to"));
            closeAll([popupId, ...opennedPopups]);
            popup.classList.add("visible");
        }

        function close(popupId) {
            let popup = document.querySelector(`.popup[data-attach-to="${popupId}"]`);
            popup.classList.remove("visible");
        }

        function init() {
            popups.forEach(popup => {
                const popupId = popup.getAttribute("data-attach-to");
                popup.addEventListener("click", (e) => {
                    if (e.target === popup) {
                        close(popupId)
                    }
                });
                popup.querySelectorAll(".popup__window-controls")
                    .forEach(el => {
                        el.addEventListener("click", () => {
                            close(popupId);
                        })
                    });

                const el = document.getElementById(popupId);
                el.classList.add("popup-oppener");
                el.addEventListener("click", e => {
                    e.preventDefault();
                    open(popupId)
                });
                el.querySelectorAll("a, .button, .clickable")
                    .forEach(clickable => {
                        if (!clickable.classList.contains("popup-oppener")) {
                            clickable.addEventListener("click", e => {
                                e.stopPropagation();
                            });
                        }
                    });
            });
        }

        init();


    });
