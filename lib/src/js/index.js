window.addEventListener("DOMContentLoaded", () => {
    window.dispatchEvent(new CustomEvent("startViewy", {
        detail: {
            root: document
        }
    }));
});