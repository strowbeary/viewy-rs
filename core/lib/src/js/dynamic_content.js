window.addEventListener("load", () => {
    document.querySelectorAll(`.dynamic-content`)
        .forEach(dynamicContent => {
            dynamicContent.addEventListener("dynamicContentLoaded", () => {
                console.log("dynamicContentLoaded");
            });
        });
});