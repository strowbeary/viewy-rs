window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".snackbar")
        .forEach(snackbar => {
            snackbar.querySelector(".button[data-snackbar-closing-button]")
                .addEventListener("click", e => {
                    snackbar.classList.add("snackbar--hidden");
                });
        });
});