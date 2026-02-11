function findAll(root, selector) {
  const matches = [];

  if (typeof root.matches === "function" && root.matches(selector)) {
    matches.push(root);
  }

  return matches.concat(Array.from(root.querySelectorAll(selector)));
}

function getAssociatedForm(control) {
  const formId = control.getAttribute("form");
  if (formId) {
    return document.getElementById(formId);
  }

  if (control.form) {
    return control.form;
  }

  return control.closest("form");
}

function submitIfNeeded(control) {
  if (!control.hasAttribute("data-auto-submit")) {
    return;
  }

  const form = getAssociatedForm(control);
  if (!form) {
    return;
  }

  if (typeof form.requestSubmit === "function") {
    form.requestSubmit();
  } else {
    form.submit();
  }
}

function getAllOptions(selectRoot) {
  return Array.from(selectRoot.querySelectorAll(".select__option"));
}

function getVisibleOptions(selectRoot) {
  return getAllOptions(selectRoot).filter((option) => !option.hidden);
}

function closeAllSelects(except) {
  document
    .querySelectorAll('.select[data-v-select-open="true"]')
    .forEach((selectRoot) => {
      if (except && selectRoot === except) {
        return;
      }

      if (typeof selectRoot.__vCloseSelect === "function") {
        selectRoot.__vCloseSelect(false);
      }
    });
}

let didRegisterGlobalSelectHandlers = false;

function ensureGlobalSelectHandlers() {
  if (didRegisterGlobalSelectHandlers) {
    return;
  }

  didRegisterGlobalSelectHandlers = true;

  document.addEventListener("click", (event) => {
    const container = event.target.closest(".select");
    closeAllSelects(container);
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closeAllSelects();
    }
  });
}

function initSelect(selectRoot) {
  if (selectRoot.dataset.vSelectInit === "true") {
    return;
  }

  const trigger = selectRoot.querySelector(".select__trigger");
  const panel = selectRoot.querySelector(".select__panel");
  const listbox = selectRoot.querySelector(".select__listbox");
  const valueDisplay = selectRoot.querySelector(".select__value");
  const hiddenField = selectRoot.querySelector(".select__field");
  const searchField = selectRoot.querySelector(".select__search");

  if (!trigger || !panel || !listbox || !valueDisplay || !hiddenField) {
    return;
  }

  selectRoot.dataset.vSelectInit = "true";

  const setActive = (option) => {
    const allOptions = getAllOptions(selectRoot);
    allOptions.forEach((opt) => {
      opt.classList.remove("is-active");
      opt.setAttribute("tabindex", "-1");
    });

    if (!option) {
      listbox.removeAttribute("aria-activedescendant");
      return;
    }

    option.classList.add("is-active");
    option.setAttribute("tabindex", "0");

    const optionId = option.getAttribute("id");
    if (optionId) {
      listbox.setAttribute("aria-activedescendant", optionId);
    }
  };

  const selectOption = (option, emitChange = true) => {
    if (!option) {
      return;
    }

    const allOptions = getAllOptions(selectRoot);
    allOptions.forEach((opt) => {
      opt.classList.remove("is-selected");
      opt.classList.remove("active");
      opt.setAttribute("aria-selected", "false");
    });

    option.classList.add("is-selected");
    option.classList.add("active");
    option.setAttribute("aria-selected", "true");

    const label = option.dataset.label || option.textContent.trim();
    const value = option.dataset.value || "";

    valueDisplay.textContent = label;
    hiddenField.value = value;
    setActive(option);

    const visibleOptions = getVisibleOptions(selectRoot);
    const index = visibleOptions.indexOf(option);
    if (index >= 0) {
      trigger.setAttribute(
        "aria-label",
        `${label}, listbox ${index + 1} of ${visibleOptions.length}`,
      );
    } else {
      trigger.setAttribute("aria-label", label);
    }

    if (emitChange) {
      hiddenField.dispatchEvent(new Event("change", { bubbles: true }));
      submitIfNeeded(hiddenField);
    }
  };

  const filterOptions = () => {
    const keyword = (searchField ? searchField.value : "").trim().toLowerCase();

    getAllOptions(selectRoot).forEach((option) => {
      if (!keyword) {
        option.hidden = false;
        return;
      }

      const label = (
        option.dataset.label ||
        option.textContent ||
        ""
      ).toLowerCase();
      option.hidden = !label.includes(keyword);
    });

    selectRoot.querySelectorAll(".select__group").forEach((group) => {
      const visibleInGroup = group.querySelector(
        ".select__option:not([hidden])",
      );
      group.hidden = !visibleInGroup;
    });

    const active = listbox.querySelector(".select__option.is-active");
    if (active && active.hidden) {
      setActive(getVisibleOptions(selectRoot)[0] || null);
    }
  };

  const openPanel = () => {
    if (trigger.disabled) {
      return;
    }

    closeAllSelects(selectRoot);
    panel.removeAttribute("hidden");
    trigger.setAttribute("aria-expanded", "true");
    selectRoot.setAttribute("data-v-select-open", "true");

    if (searchField) {
      searchField.focus();
      searchField.select();
      return;
    }

    const active = listbox.querySelector(".select__option.is-active");
    if (active) {
      active.focus();
    }
  };

  const closePanel = (restoreFocus = false) => {
    panel.setAttribute("hidden", "hidden");
    trigger.setAttribute("aria-expanded", "false");
    selectRoot.setAttribute("data-v-select-open", "false");

    if (restoreFocus) {
      trigger.focus();
    }
  };

  const isOpen = () => trigger.getAttribute("aria-expanded") === "true";

  const moveActive = (step) => {
    const visibleOptions = getVisibleOptions(selectRoot);
    if (visibleOptions.length === 0) {
      return;
    }

    const active = listbox.querySelector(".select__option.is-active");
    const currentIndex = active ? visibleOptions.indexOf(active) : -1;

    const nextIndex =
      currentIndex < 0
        ? 0
        : Math.max(0, Math.min(visibleOptions.length - 1, currentIndex + step));

    const next = visibleOptions[nextIndex];
    setActive(next);
    next.scrollIntoView({ block: "nearest" });
  };

  const activateBoundary = (first) => {
    const visibleOptions = getVisibleOptions(selectRoot);
    if (visibleOptions.length === 0) {
      return;
    }

    const option = first
      ? visibleOptions[0]
      : visibleOptions[visibleOptions.length - 1];

    setActive(option);
    option.scrollIntoView({ block: "nearest" });
  };

  const commitActiveOption = () => {
    const active = listbox.querySelector(".select__option.is-active");
    if (!active) {
      return;
    }

    selectOption(active, true);
    closePanel(true);
  };

  const handleKeyboard = (event) => {
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        if (!isOpen()) {
          openPanel();
        }
        moveActive(1);
        break;
      case "ArrowUp":
        event.preventDefault();
        if (!isOpen()) {
          openPanel();
        }
        moveActive(-1);
        break;
      case "Home":
        event.preventDefault();
        if (!isOpen()) {
          openPanel();
        }
        activateBoundary(true);
        break;
      case "End":
        event.preventDefault();
        if (!isOpen()) {
          openPanel();
        }
        activateBoundary(false);
        break;
      case "Enter":
      case " ":
        event.preventDefault();
        if (!isOpen()) {
          openPanel();
        } else {
          commitActiveOption();
        }
        break;
      case "Escape":
        if (isOpen()) {
          event.preventDefault();
          closePanel(true);
        }
        break;
      case "Tab":
        if (isOpen()) {
          closePanel(false);
        }
        break;
      default:
        break;
    }
  };

  selectRoot.__vCloseSelect = closePanel;

  const initialSelected =
    selectRoot.querySelector('.select__option[aria-selected="true"]') ||
    getVisibleOptions(selectRoot)[0] ||
    null;

  if (initialSelected) {
    selectOption(initialSelected, false);
  }

  trigger.addEventListener("click", () => {
    if (isOpen()) {
      closePanel(false);
    } else {
      openPanel();
    }
  });

  trigger.addEventListener("keydown", handleKeyboard);
  listbox.addEventListener("keydown", handleKeyboard);

  if (searchField) {
    searchField.addEventListener("keydown", (event) => {
      if (event.key === "Enter") {
        event.preventDefault();
        commitActiveOption();
        return;
      }

      if (event.key === "ArrowDown") {
        event.preventDefault();
        moveActive(1);
      }

      if (event.key === "ArrowUp") {
        event.preventDefault();
        moveActive(-1);
      }
    });

    searchField.addEventListener("input", () => {
      filterOptions();

      const selectedVisible = selectRoot.querySelector(
        '.select__option[aria-selected="true"]:not([hidden])',
      );
      setActive(selectedVisible || getVisibleOptions(selectRoot)[0] || null);
    });
  }

  getAllOptions(selectRoot).forEach((option) => {
    option.addEventListener("click", (event) => {
      event.preventDefault();
      selectOption(option, true);
      closePanel(true);
    });

    option.addEventListener("mousemove", () => {
      if (isOpen()) {
        setActive(option);
      }
    });
  });
}

function initPickerAutoSubmit(root) {
  findAll(root, ".picker .picker__item-input[data-auto-submit]").forEach(
    (input) => {
      if (input.dataset.vPickerInit === "true") {
        return;
      }

      input.dataset.vPickerInit = "true";
      input.addEventListener("change", () => {
        submitIfNeeded(input);
      });
    },
  );
}

export function init(root) {
  ensureGlobalSelectHandlers();

  findAll(root, ".select").forEach((selectRoot) => {
    initSelect(selectRoot);
  });

  initPickerAutoSubmit(root);
}
