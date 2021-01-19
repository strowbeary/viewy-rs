(() => {
    window.addEventListener("load", () => {
        document.queryAll("button.button")
        .forEach(button => button.addEventListener("click", () => console.log("button click !")))
    })
})();