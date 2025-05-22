window.addEventListener("startViewy", ({detail}) => {
    const selectAllCheckbox = detail.root.querySelectorAll("table .select-all input[type='checkbox']");
    selectAllCheckbox.forEach(checkbox => {
        const bodyCheckboxes =  checkbox.closest("form").querySelectorAll("tbody input[type='checkbox']")
        checkbox.addEventListener("change", e => {
            bodyCheckboxes.forEach(bodyCheckbox => {
                bodyCheckbox.checked = checkbox.checked;
            });
        });

    });
});
