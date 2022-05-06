/***
 * Selectable row mechanism
 */

window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll("table input[type='checkbox']")
        .forEach(checkbox => {
            let parent = checkbox;
            while (parent.tagName !== "TR") {
                parent = parent.parentElement;
            }
            parent.classList.toggle("selected", checkbox.checked);
            checkbox.addEventListener("change", e => {
                parent.classList.toggle("selected", e.target.checked);
            })
        });
});