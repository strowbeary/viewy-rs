window.addEventListener("load", e => {
    document.querySelectorAll(".snackbar")
        .forEach(snackbar => {
            snackbar.querySelector(".button[data-snackbar-closing-button]")
                .addEventListener("click", e => {
                    snackbar.classList.add("snackbar--hidden");
                });
        });
});