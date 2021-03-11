(() => {
    window.addEventListener("load", () => {
        document.querySelectorAll("button.button")
        .forEach(button => button.addEventListener("click", () => console.log("button click !")))
    })
})();