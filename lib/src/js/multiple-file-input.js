window.addEventListener("startViewy", ({detail}) => {

    detail.root.querySelectorAll(".multiple-file-input")
        .forEach(fileInputComponent => {
            let triggerButton = fileInputComponent.querySelector(".multiple-file-input__button");
            if (triggerButton) {
                triggerButton.addEventListener("click", e => {
                    e.preventDefault();
                    fileInputComponent.querySelector("input[type='file']")
                        .click();
                });
            }
        });
    detail.root.querySelectorAll(".multiple-file-input")
        .forEach(fileInputComponent => {
            let input;
            if (fileInputComponent.classList.contains("fileInputComponent__input")) {
                input = fileInputComponent;
            } else {
                input = fileInputComponent.querySelector("input[type='file']");
            }

            let form = fileInputComponent.closest("form");

            async function uploadFile(files) {

                let formData = new FormData(form);
                formData.delete(input.name);

                await Promise.all(files.map(([id, file]) =>  new Promise((resolve, reject) => {
                    let xhr = new XMLHttpRequest();
                    xhr.open(form.method, form.action, true);
                    formData.set("file", file);

                    const progressBar = document.querySelector(`[data-file-id="${id}"] progress`);
                    progressBar.removeAttribute("value");
                    xhr.upload.addEventListener("progress", ({lengthComputable, loaded, total}) => {
                        console.log(lengthComputable, loaded, total);
                        if (lengthComputable) {

                            progressBar.max = total;
                            progressBar.value = loaded;
                        }
                    });
                    xhr.addEventListener("load", resolve);
                    xhr.addEventListener("error", reject);
                    xhr.send(formData);
                })));

            }

            let instance_id = fileInputComponent.getAttribute("data-file-list");
            let fileInput;
            if (fileInputComponent.classList.contains("multiple-file-input--hidden")) {
                fileInput = fileInputComponent;
            } else {
                fileInput = fileInputComponent.querySelector("input[type='file']");
            }
            let file_list = document.getElementById(instance_id);
            let template = file_list.querySelector("template").content;
            fileInput.addEventListener("change", e => {
                file_list.innerHTML = "";
                file_list.style.display = "flex";
                let files = [...e.target.files]
                    .map((file) => [crypto.randomUUID(), file]);

                files.forEach(([id, file]) => {
                    let template_instance = template.cloneNode(true);
                    template_instance.querySelector(".multiple-file-input__file-list__item").setAttribute("data-file-id", id);
                    template_instance.querySelector(".t-file-name").textContent = file.name;
                    file_list.appendChild(template_instance);
                });
                uploadFile(files);

            });
        });

});