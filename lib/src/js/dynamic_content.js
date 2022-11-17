window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(`.dynamic-content`)
        .forEach(dynamicContent => {
            dynamicContent.addEventListener("dynamicContentLoaded", () => {
                console.log("dynamicContentLoaded");
            });
        });
});