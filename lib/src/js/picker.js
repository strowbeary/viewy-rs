function viewySelect(picker) {
    const dropdownId = picker.querySelector(".picker--dropdown__input").id;
    const input = picker.querySelector(".picker--dropdown__input");
    const field = picker.querySelector(".picker--dropdown__input__field");
    const valueDisplay = picker.querySelector(".picker--dropdown__input__value-display");

    const dropdown = document.querySelector(`.picker--dropdown__dropdown[data-attach-to="${dropdownId}"]`);
    const searchbar = dropdown.querySelector(".picker--dropdown__dropdown__search-bar");
    const originalOptions = [...dropdown.querySelectorAll(".picker--dropdown__dropdown__option-list__option")];
    let options = originalOptions;
    let currentValue = null;

    searchbar.addEventListener("input", e => {
        const keyword = e.target.value;
        options = originalOptions.filter(opt => {
            if (keyword.length > 0) {
                if (!opt.textContent.toLowerCase().includes(keyword.toLowerCase())) {
                    opt.style.display = "none";
                    return false;
                } else {
                    opt.style.display = null;
                    return true;
                }
            } else {
                opt.style.display = null;
                return true;
            }
        });
        dropdown.popperInstance.forceUpdate();
    });


    function setChecked(radioButton) {
        radioButton.checked = true;

        radioButton.parentElement.scrollIntoView();
        const has_changed = setValue(radioButton);
        if (has_changed) {
            field.dispatchEvent(new CustomEvent('change'));
        }
    }

    function setValue(newValue) {
        if (currentValue == newValue) return false;

        const pos = options.indexOf(newValue.parentNode) + 1;
        valueDisplay.textContent = newValue.parentNode.textContent;
        input.setAttribute('aria-label', `${newValue.parentNode.textContent}, listbox ${pos} of ${options.length}`);
        currentValue = newValue;
        field.value = newValue.parentNode.querySelector("input[type='radio']").value;

        originalOptions.forEach(opt => {
            opt.classList.remove('active');
            opt.setAttribute('aria-selected', 'false');
        })
        newValue.parentNode.classList.add('active');
        newValue.parentNode.setAttribute('aria-selected', 'true');
        return true;
    }

    function keyboardController(e) {
        const keycode = e.which;
        console.log(keycode);
        const current = options.indexOf(dropdown.querySelector('.active'));
        switch (keycode) {
            case 27: // ESC
                e.preventDefault();
                dropdown.removeAttribute("data-show");
                break;
            case 35: // END
                e.preventDefault();
                dropdown.setAttribute("data-show", "data-show");
                setChecked(options[options.length - 1].querySelector('input[type="radio"]'))

                break;
            case 36: // HOME
                e.preventDefault();
                dropdown.setAttribute("data-show", "data-show");
                setChecked(options[0].querySelector('input[type="radio"]'))
                break;
            case 38: // UP
                e.preventDefault();
                dropdown.setAttribute("data-show", "data-show");
                setChecked(options[current > 0 ? current - 1 : 0].querySelector('input[type="radio"]'));
                break;
            case 40: // DOWN
                e.preventDefault();
                dropdown.setAttribute("data-show", "data-show");
                setChecked(options[current < options.length - 1 ? current + 1 : options.length - 1].querySelector('[type="radio"]'));

                break;
            case 13: // ENTER
                e.preventDefault();
                dropdown.toggleAttribute("data-show");
                setChecked(options[current < options.length - 1 ? current + 1 : options.length - 1].querySelector('[type="radio"]'));

                break;
            default:
                break;
        }

    }


    input.setAttribute('aria-haspopup', 'listbox');
    input.setAttribute('aria-label', `unselected listbox`);
    input.setAttribute('aria-live', `polite`);

    dropdown.querySelector(".picker--dropdown__dropdown__option-list").setAttribute('role', 'listbox');

    options.forEach(opt => {
        opt.setAttribute('role', 'option');
        opt.addEventListener("change", e => {
            setChecked(e.target);
        });
        opt.addEventListener('mousedown', () => {
            setChecked(opt.querySelector("input[type='radio']"));
        });
        opt.addEventListener('mouseup', () => {
            dropdown.removeAttribute("data-show");
        });
    });
    setValue(dropdown.querySelector(".picker--dropdown__dropdown__option-list__option input[checked]"));

    input.addEventListener('keydown', keyboardController);
    dropdown.addEventListener('keydown', keyboardController);
}

window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".picker--dropdown")
        .forEach((picker) => {
            viewySelect(picker);
        });
});