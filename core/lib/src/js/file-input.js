window.addEventListener("load", () => {
    document.querySelectorAll("[data-input-file]")
        .forEach(fileInputOpener => {
            if (fileInputOpener) {
                fileInputOpener.addEventListener("click", (e) => {
                    e.preventDefault();

                    let fileInput = document.getElementById(fileInputOpener.getAttribute("data-input-file"));

                    if (fileInput.hasAttribute("data-auto-submit")) {
                        fileInput.addEventListener("change", () => {
                            fileInput.closest("form").submit();
                        });
                    }

                    fileInput.click();
                });
            }
        });
    document.querySelectorAll(".file-input")
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
    document.querySelectorAll(".file-input--simple")
        .forEach(fileInputComponent => {
            let fileInput = fileInputComponent.querySelector("input[type='file']");
            fileInput.addEventListener("change", e => {
                fileInputComponent.querySelector(".file-input__file-name").textContent = [...e.target.files].map(file => file.name).join(", ");
            });
        });

    document.querySelectorAll(".file-input--image")
        .forEach(fileInputComponent => {
            let fileInput = fileInputComponent.querySelector("input[type='file']");
            fileInput.addEventListener("change", e => {
                fileInputComponent.querySelector(".file-input__file-name").textContent = [...e.target.files].map(file => file.name).join(", ");
                fileInputComponent.querySelector(".file-input__image-preview").src = URL.createObjectURL(e.target.files[0]);
            });
        });
});