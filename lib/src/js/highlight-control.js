window.addEventListener("startViewy", ({detail}) => {
    console.log("highlight-control");

    detail.root.querySelectorAll(".card[data-remove-highlight-on-submit]")
        .forEach((card) => {
            const form_name = card.getAttribute("data-remove-highlight-on-submit");

            function remove_highlight(e) {
                card.classList.remove("card--highlighted");
            }

            const form = document.querySelector(`form#${form_name}`);
            form.addEventListener("submit", remove_highlight);
            form.addEventListener("asyncSubmit", remove_highlight);
        });

    detail.root.querySelectorAll(".badge[data-remove-on-click]")
        .forEach((badge) => {
            const element_id = badge.getAttribute("data-remove-on-click");

            function remove_badge() {
               badge.remove();
            }

            const form = document.querySelector(`#${element_id}`);
            form.addEventListener("click", remove_badge);
        });
});