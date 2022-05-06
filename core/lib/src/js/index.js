window.addEventListener("load", () => {
    window.dispatchEvent(new CustomEvent("startViewy", {
        detail: {
            root: document
        }
    }));
});