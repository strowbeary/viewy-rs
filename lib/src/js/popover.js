window.addEventListener("startViewy", ({detail}) => {
    const popovers = detail.root.querySelectorAll(".popover");

    function closeAll(except) {
        popovers.forEach(popover => {
            if (except) {
                if (popover.getAttribute("data-attach-to") !== except.getAttribute("data-attach-to")){
                    popover.removeAttribute("data-show");
                }
            } else {
                popover.removeAttribute("data-show");
            }
        });
    }

    window.addEventListener("viewy-popup-open", e => {
        closeAll()
    });

    function init() {
        popovers.forEach(popover => {
            const el = document.getElementById(popover.getAttribute("data-attach-to"));
            let popperInstance = Popper.createPopper(el, popover, {
                placement: popover.getAttribute("data-placement"),
                modifiers: [
                    {
                        name: 'preventOverflow',
                        options: {
                            padding: 16,
                        },
                    },
                    {
                        name: 'offset',
                        options: {
                            offset: [0, 8],
                        },
                    },
                    {
                        name: 'arrow',
                        options: {
                            padding: 5, // 5px from the edges of the popper
                        },
                    },
                ],
            });
            popover.popperInstance=popperInstance;
            window.addEventListener("click", e => {
                if (!(e.target.isSameNode(popover) || popover.contains(e.target))) {
                    popover.removeAttribute("data-show");
                }
                if (e.target.isSameNode(el) || el.contains(e.target)) {
                    e.preventDefault();
                    popover.setAttribute("data-show", "data-show");
                }

                popperInstance.update();
            });
            el.addEventListener("click", e => {
                e.preventDefault();
                popover.setAttribute("data-show", "data-show");
                popperInstance.update();
            });

        });
    }

    init();
});