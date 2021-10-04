function closeAllPopover() {
    document.querySelectorAll(".popover").forEach(popover => {
        popover.removeAttribute("data-show");
    });
}

(() => {
    window.addEventListener("load", () => {
        const popups = document.querySelectorAll(".popup");

        function closeAll(except) {
            popups.forEach(popup => {
                if (popup.getAttribute("data-attach-to") !== except.getAttribute("data-attach-to"))
                    popup.classList.remove("visible");
            });
        }

        function init() {
            popups.forEach(popup => {
                popup.addEventListener("click", (e) => {
                    if (e.target === popup) {
                        popup.classList.remove("visible");
                    }
                });
                const el = document.getElementById(popup.getAttribute("data-attach-to"));
                el.addEventListener("click", e => {
                    e.preventDefault();
                    if (popup.classList.contains("visible")) {
                        popup.classList.remove("visible");
                    } else {
                        closeAll(popup);
                        closeAllPopover();
                        popup.classList.add("visible");
                    }
                });
            });
        }

        init();
    });
})();