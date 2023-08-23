function setErrorHandling(component_root, input) {
    if (input) {
        input.addEventListener("invalid", e => {
            e.preventDefault();
            let old_helper_text = component_root.querySelector(".file-input__helper-text");
            if (old_helper_text !== null) {
                old_helper_text.remove();
            }
            let helper_text = document.createElement("div");
            helper_text.classList.add("view", "text", "text--caption", "file-input__helper-text");
            helper_text.textContent = "Vous devez selectionner un fichier";
            component_root.appendChild(helper_text);
            component_root.classList.add("file-input--error");
            input.addEventListener("change", () => {
                component_root.classList.remove("file-input--error");
                if (old_helper_text !== null) {
                    helper_text.replaceWith(old_helper_text);
                } else {
                    helper_text.remove();
                }
            }, {once: true});
        });
    }
}

window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll("[data-input-file]")
        .forEach(fileInputOpener => {
                fileInputOpener.addEventListener("click", (e) => {
                    e.preventDefault();
                    e.stopPropagation();

                    let fileInput = document.getElementById(fileInputOpener.getAttribute("data-input-file"));

                    if (fileInput.hasAttribute("data-auto-submit")) {
                        fileInput.addEventListener("change", () => {
                            fileInput.closest("form").submit();
                        });
                    }

                    fileInput.click();
                });

        });
    detail.root.querySelectorAll(".file-input")
        .forEach(fileInputComponent => {
            let triggerButton = fileInputComponent.querySelector(".file-input__button");
            if (triggerButton) {
                triggerButton.addEventListener("click", e => {
                    e.preventDefault();
                    fileInputComponent.querySelector("input[type='file']")
                        .click();
                });
            }
        });
    detail.root.querySelectorAll(".file-input--simple")
        .forEach(fileInputComponent => {
            let fileInput = fileInputComponent.querySelector("input[type='file']");
            fileInput.addEventListener("change", e => {
                fileInputComponent.querySelector(".file-input__file-name").textContent = [...e.target.files].map(file => file.name).join(", ");
            });
            setErrorHandling(fileInputComponent, fileInput);
        });

    detail.root.querySelectorAll(".file-input--image")
        .forEach(fileInputComponent => {
            let fileInput = fileInputComponent.querySelector("input[type='file']");
            fileInput.addEventListener("change", e => {
                fileInputComponent.querySelector(".file-input__file-name").textContent = [...e.target.files].map(file => file.name).join(", ");
                fileInputComponent.querySelector(".file-input__image-preview").src = URL.createObjectURL(e.target.files[0]);
            });
            setErrorHandling(fileInputComponent, fileInput);
        });
});