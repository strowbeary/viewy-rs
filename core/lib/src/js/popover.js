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
                window.addEventListener("click", e => {
                    console.log("click on", e.target);
                    if (!(e.target.isSameNode(popover) || popover.contains(e.target))) {
                        popover.removeAttribute("data-show");
                    }
                    if(e.target.isSameNode(el) || el.contains(e.target)) {
                        popover.setAttribute("data-show", "data-show");
                    }

                    popperInstance.update();
                });

            });
        }
        init();
    });
})();