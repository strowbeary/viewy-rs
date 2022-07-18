function viewySelect(picker) {
    const dropdownId = picker.querySelector(".picker--dropdown__input").id;
    const input = picker.querySelector(".picker--dropdown__input");
    const valueDisplay = picker.querySelector(".picker--dropdown__input__value-display");

    const dropdown = document.querySelector(`.picker--dropdown__dropdown[data-attach-to="${dropdownId}"]`);
    const searchbar = dropdown.querySelector(".picker--dropdown__dropdown__search-bar");
    const options = dropdown.querySelectorAll(".picker--dropdown__dropdown__option-list__option");
    let currentValue = null;
    let mouseDown = false;

    searchbar.addEventListener("input", e => {
        const keyword = e.target.value;
        options.forEach(opt => {
            if (keyword.length > 0) {

                if (!opt.textContent.toLowerCase().includes(keyword.toLowerCase())) {
                    opt.style.display = "none";
                } else {
                    opt.style.display = null;
                }
            } else {
                opt.style.display = null;
            }
        })
    });


    function setChecked(radioButton) {
        radioButton.checked = true;
        setValue(radioButton)
    }
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

    function keyboardController(e) {
            const keycode = e.which;
            const current = [...options].indexOf(dropdown.querySelector('.active'));
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
                    break;
                case 32: // SPACE
                    e.preventDefault();
                    dropdown.toggleAttribute("data-show");
                    break;
                default:break;
            }

    }


    input.setAttribute('aria-haspopup', 'listbox');
    input.setAttribute('aria-label', `unselected listbox`);
    input.setAttribute('aria-live', `polite`);

    dropdown.querySelector(".picker--dropdown__dropdown__option-list").setAttribute('role', 'listbox');

    options.forEach(opt => {
        opt.setAttribute('role', 'option');
        opt.addEventListener("change", e => {
            setValue(e.target);
        });
        opt.addEventListener('mousedown', () => {
            mouseDown = true;
        })
        opt.addEventListener('mouseup', () => {
            mouseDown = false;
            dropdown.removeAttribute("data-show");
        })
    });

    setValue(dropdown.querySelector(".picker--dropdown__dropdown__option-list__option input:checked"));

    input.addEventListener('keydown', keyboardController);
    dropdown.addEventListener('keydown', keyboardController);
}

window.addEventListener("startViewy", ({detail}) => {
    document.querySelectorAll(".picker--dropdown")
        .forEach((picker) => {
            viewySelect(picker);
        });
});