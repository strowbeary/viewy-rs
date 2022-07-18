function viewySelect(picker) {
    const dropdownId = picker.querySelector(".picker--dropdown__input").id;
    const input = picker.querySelector(".picker--dropdown__input");
    const field = picker.querySelector(".picker--dropdown__input__field");
    const valueDisplay = picker.querySelector(".picker--dropdown__input__value-display");

    const dropdown = document.querySelector(`.picker--dropdown__dropdown[data-attach-to="${dropdownId}"]`);
    const options = dropdown.querySelectorAll(".picker--dropdown__dropdown__option-list__option");
    let currentValue = null;


    setValue(dropdown.querySelector(".picker--dropdown__dropdown__option-list__option input:checked"));

    function setValue(newValue) {
        if (currentValue == newValue) return;

        const pos = [...options].indexOf(newValue.parentNode) + 1;
        valueDisplay.textContent = newValue.parentNode.textContent;
        input.setAttribute('aria-label', `${newValue.parentNode.textContent}, listbox ${pos} of ${options.length}`);
        currentValue = newValue;

        options.forEach(opt => {
            opt.classList.remove('active');
            opt.setAttribute('aria-selected', 'false');
        })
        newValue.parentNode.classList.add('active');
        newValue.parentNode.setAttribute('aria-selected', 'true');
    }
}

window.addEventListener("startViewy", ({detail}) => {
    document.querySelectorAll(".picker--dropdown")
        .forEach((picker) => {
            viewySelect(picker);
        });
});