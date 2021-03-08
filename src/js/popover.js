;(() => {
    window.addEventListener("load", () => {
        document.querySelectorAll(".popover")
            .forEach(popover => {
                const el = document.getElementById(popover.getAttribute("data-attach-to"));
                let popperInstance = Popper.createPopper(el, popover, {
                    placement: "right",
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
                    popover.style.display = popover.style.display === "none" ? "block" : "none";
                    popperInstance.update();
                });

            });
    });
})();