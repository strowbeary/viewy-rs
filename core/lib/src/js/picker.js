const viewySelect = (component) => {
    const details = component.querySelector(".picker--dropdown__input");
    const summary = component.querySelector(".picker--dropdown__input__summary");
    const valueDisplay = component.querySelector(".picker--dropdown__input__summary .text");
    const options = component.querySelectorAll(".picker--dropdown__input__dropdown__option-list__option");
    let value = component.querySelector('.picker--dropdown__input__dropdown__option-list__option input:checked');
    let mouseDown = false;
    setValue(value);


    function updateValue(e) {
        const that = details.querySelector('input[type="radio"]:checked');
        if (!that) return;
        setValue(that)
    }

    function setChecked(that) {
        that.checked = true;
        setValue(that)
    }

    function setValue(newValue) {
        if (value == newValue) return;

        const pos = [...options].indexOf(newValue.parentNode) + 1;
        valueDisplay.textContent = newValue.parentNode.textContent;
        summary.setAttribute('aria-label', `${newValue}, listbox ${pos} of ${options.length}`);
        value = newValue;

        options.forEach(opt => {
            opt.classList.remove('active');
            opt.setAttribute('aria-selected', 'false');
        })
        newValue.parentNode.classList.add('active');
        newValue.parentNode.setAttribute('aria-selected', 'true');

        details.dispatchEvent(new Event('change'));
    }


    details.addEventListener('toggle', () => {
        if (details.open) return;
        updateValue();
    });

    window.addEventListener("click", e => {
        if (!(e.target.isSameNode(details) || details.contains(e.target))) {
            details.removeAttribute("open");
        }
    });

    options.forEach(opt => {
        opt.addEventListener('mousedown', () => {
            mouseDown = true;
        })
        opt.addEventListener('mouseup', () => {
            mouseDown = false;
            details.removeAttribute('open');
        })
    })

    details.addEventListener('keyup', e => {
        const keycode = e.which;
        const current = [...options].indexOf(details.querySelector('.active'));
        switch (keycode) {
            case 27: // ESC
                e.preventDefault()
                details.removeAttribute('open');
                break;
            case 35: // END
                e.preventDefault();
                if (!details.open) details.setAttribute('open', '');
                setChecked(options[options.length - 1].querySelector('input[type="radio"]'))
                break;
            case 36: // HOME
                e.preventDefault();
                if (!details.open) details.setAttribute('open', '');
                setChecked(options[0].querySelector('input[type="radio"]'))
                break;
            case 38: // UP
                e.preventDefault();
                if (!details.open) details.setAttribute('open', '');
                setChecked(options[current > 0 ? current - 1 : 0].querySelector('input[type="radio"]'));
                break;
            case 40: // DOWN
                e.preventDefault();
                if (!details.open) details.setAttribute('open', '');
                setChecked(options[current < options.length - 1 ? current + 1 : options.length - 1].querySelector('[type="radio"]'));
                break;
        }
    });

    details.setAttribute('aria-haspopup', 'listbox');
    details.querySelector('.picker--dropdown__input__dropdown__option-list').setAttribute('role', 'listbox');

    summary.setAttribute('aria-label', `unselected listbox`);
    summary.setAttribute('aria-live', `polite`);
    options.forEach(opt => {
        opt.setAttribute('role', 'option');
    });
}

window.addEventListener("startViewy", ({detail}) => {
    document.querySelectorAll(".picker--dropdown")
        .forEach((picker) => {
            viewySelect(picker);
        });
});



