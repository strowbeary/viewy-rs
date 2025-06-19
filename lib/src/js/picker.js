function viewySelect(detail, picker) {
  const input = picker.querySelector(".picker--dropdown__input");
  const valueDisplay = picker.querySelector(
    ".picker--dropdown__input__value-display",
  );
  const dropdownId = input.id;
  const form = valueDisplay.closest("form");

  const dropdown = document.querySelector(
    `.picker--dropdown__dropdown[data-attach-to="${dropdownId}"]`,
  );
  const searchbar = dropdown.querySelector(
    ".picker--dropdown__dropdown__search-bar",
  );
  const optionElements = Array.from(
    dropdown.querySelectorAll(
      ".picker--dropdown__dropdown__option-list__option",
    ),
  );

  const isMultiple = dropdown.querySelector("input[type='checkbox']") !== null;
  let currentValues = isMultiple ? [] : null;

  // Accessibility attributes
  input.setAttribute("aria-haspopup", "listbox");
  input.setAttribute("aria-label", "unselected listbox");
  input.setAttribute("aria-live", "polite");

  dropdown
    .querySelector(".picker--dropdown__dropdown__option-list")
    .setAttribute("role", "listbox");

  // Search filtering
  searchbar.addEventListener("input", (e) => {
    const keyword = e.target.value.toLowerCase();
    optionElements.forEach((opt) => {
      const match = opt.textContent.toLowerCase().includes(keyword);
      opt.style.display = match ? "" : "none";
    });
    dropdown.popperInstance?.forceUpdate?.();
  });

  function updateDisplay(values) {
    const labels = values.map((input) => input.parentNode.textContent.trim());
    valueDisplay.textContent = labels.join(", ");
    input.setAttribute("aria-label", `${labels.join(", ")}, listbox`);
  }

  function setValue(inputEl) {
    const parent = inputEl.closest(
      ".picker--dropdown__dropdown__option-list__option",
    );

    if (isMultiple) {
      const isChecked = inputEl.checked;
      if (isChecked) {
        if (!currentValues.includes(inputEl)) currentValues.push(inputEl);
      } else {
        currentValues = currentValues.filter((i) => i !== inputEl);
      }
    } else {
      if (currentValues === inputEl) return false;
      currentValues = inputEl;

      // Uncheck all other radios
      optionElements.forEach((opt) => {
        const radio = opt.querySelector("input[type='radio']");
        if (radio && radio !== inputEl) radio.checked = false;
      });
    }

    // Update UI state
    optionElements.forEach((opt) => {
      opt.classList.remove("active");
      opt.setAttribute("aria-selected", "false");
    });

    const activeInputs = isMultiple ? currentValues : [inputEl];
    activeInputs.forEach((input) => {
      const opt = input.closest(
        ".picker--dropdown__dropdown__option-list__option",
      );
      opt.classList.add("active");
      opt.setAttribute("aria-selected", "true");
    });

    updateDisplay(activeInputs);
    return true;
  }

  function handleKeyboard(e) {
    const key = e.which;
    const activeIndex = optionElements.indexOf(
      dropdown.querySelector(".active"),
    );
    const lastIndex = optionElements.length - 1;

    const focusOption = (index) => {
      const opt = optionElements[Math.max(0, Math.min(index, lastIndex))];
      const input = opt.querySelector("input");
      setChecked(input);
    };

    switch (key) {
      case 27: // ESC
        e.preventDefault();
        dropdown.removeAttribute("data-show");
        break;
      case 35: // END
        e.preventDefault();
        dropdown.setAttribute("data-show", "data-show");
        focusOption(lastIndex);
        break;
      case 36: // HOME
        e.preventDefault();
        dropdown.setAttribute("data-show", "data-show");
        focusOption(0);
        break;
      case 38: // UP
        e.preventDefault();
        dropdown.setAttribute("data-show", "data-show");
        focusOption(activeIndex > 0 ? activeIndex - 1 : 0);
        break;
      case 40: // DOWN
        e.preventDefault();
        dropdown.setAttribute("data-show", "data-show");
        focusOption(activeIndex < lastIndex ? activeIndex + 1 : lastIndex);
        break;
      case 13: // ENTER
        e.preventDefault();
        dropdown.toggleAttribute("data-show");
        break;
    }
  }

  // Setup each option
  optionElements.forEach((opt) => {
    const inputEl = opt.querySelector("input");

    if (form) inputEl.setAttribute("form", form.id);
    opt.setAttribute("role", "option");

    // Lorsqu'un input change (clic ou clavier), on met à jour l'affichage
    inputEl.addEventListener("change", () => {
      setValue(inputEl);
      valueDisplay.dispatchEvent(new CustomEvent("change"));
    });

    // Clique sur l’option : ne rien forcer, le navigateur gère le toggle
    opt.addEventListener("click", (e) => e.stopPropagation());

    // Fermer dropdown sur sélection, sauf en multi
    opt.addEventListener("mouseup", () => {
      if (!isMultiple) {
        dropdown.removeAttribute("data-show");
      }
    });
  });

  // Initial value setup
  const initialInputs = dropdown.querySelectorAll("input:checked");
  initialInputs.forEach((input) => setValue(input));

  // Keyboard support
  input.addEventListener("keydown", handleKeyboard);
  dropdown.addEventListener("keydown", handleKeyboard);
}

// Initialization hook
window.addEventListener("startViewy", ({ detail }) => {
  detail.root.querySelectorAll(".picker--dropdown").forEach((picker) => {
    viewySelect(detail, picker);
  });

  detail.root.querySelectorAll(".picker--segmented").forEach((picker) => {
    const fields = picker.querySelectorAll(
      ".picker--segmented__option-list__radio",
    );
    fields.forEach((field) => {
      field.addEventListener("change", () => {
        if (field.form.hasAttribute("data-async")) {
          asyncSubmit(detail.root, field.form, null);
        } else {
          field.form.submit();
        }
      });
    });
  });
});
