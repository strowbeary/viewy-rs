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
            let attached_form = popup.getAttribute("data-form-to-submit-on-open");
            if (attached_form) {
                console.log(attached_form);
                let form = document.getElementById(attached_form);
                if (form.hasAttribute("data-async")) {
                    asyncSubmit(detail.root, form)
                } else {
                    form.submit();
                }
            }
        }

        function close(popupId) {
            let popup = document.querySelector(`.popup[data-attach-to="${popupId}"]`);
            popup.classList.remove("visible");
        }

        function init() {
            popups.forEach(popup => {
                const popupId = popup.getAttribute("data-attach-to");
                function close_with_form_check(popupId) {
                    close(popupId);
                    let attached_form = popup.getAttribute("data-form-to-submit-on-close");
                    if (attached_form) {
                        console.log(attached_form);
                        let form = document.getElementById(attached_form);
                        if (form.hasAttribute("data-async")) {
                            asyncSubmit(detail.root, form)
                        } else {
                            form.submit();
                        }
                    }
                }
                popup.addEventListener("click", (e) => {
                    if (e.target === popup) {
                        close_with_form_check(popupId)
                    }
                });
                popup.querySelectorAll(".popup__window-controls")
                    .forEach(el => {
                        el.addEventListener("click", () => {
                            close_with_form_check(popupId)
                        })
                    });

                const el = document.getElementById(popupId);
                el.addEventListener("click", e => {
                    e.preventDefault();
                    open(popupId)
                });
                el.querySelectorAll("a, .button, .clickable")
                    .forEach(clickable => {
                        if (!clickable.classList.contains("popup--opener") || !clickable.classList.contains("popover--opener")) {
                            clickable.addEventListener("click", e => {
                                e.stopPropagation();
                            });
                        }
                    });
            });
        }

        init();


    });
