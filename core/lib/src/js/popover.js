(() => {
    window.addEventListener("load", () => {
        const popovers = document.querySelectorAll(".popover");
        function closeAll(except) {
            popovers.forEach(popover => {
                if(popover.getAttribute("data-attach-to") !== except.getAttribute("data-attach-to"))
                    popover.removeAttribute("data-show");
            });
        }
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
                    ],
                });
                el.addEventListener("click", e => {
                    if (popover.getAttribute("data-show")) {
                        popover.removeAttribute("data-show");
                    } else {
                        closeAll(popover);
                        popover.setAttribute("data-show", "data-show");
                    }
                    popperInstance.update();
                });

            });
        }
        init();
    });
})();