window.addEventListener("startViewy", ({ detail }) => {
  detail.root
    .querySelectorAll(".multiple-file-input")
    .forEach((fileInputComponent) => {
      let triggerButton = fileInputComponent.querySelector(
        ".multiple-file-input__button",
      );
      if (triggerButton) {
        triggerButton.addEventListener("click", (e) => {
          e.preventDefault();
          fileInputComponent.querySelector("input[type='file']").click();
        });
      }
    });
  detail.root
    .querySelectorAll(".multiple-file-input")
    .forEach((fileInputComponent) => {
      let input;
      if (fileInputComponent.classList.contains("multiple-file-input__input")) {
        input = fileInputComponent;
      } else {
        input = fileInputComponent.querySelector("input[type='file']");
      }

      let form = fileInputComponent.closest("form");

      async function uploadFile(files) {
        let formData = new FormData(form);
        formData.delete(input.name);

        await Promise.all(
          files.map(
            ([id, file]) =>
              new Promise((resolve, reject) => {
                let xhr = new XMLHttpRequest();
                xhr.open(form.method, form.action, true);
                formData.set(input.name, file);

                const progressBar = document.querySelector(
                  `[data-file-id="${id}"] progress`,
                );
                progressBar.removeAttribute("value");
                xhr.upload.addEventListener(
                  "progress",
                  ({ lengthComputable, loaded, total }) => {
                    console.log(lengthComputable, loaded, total);
                    if (lengthComputable) {
                      progressBar.max = total;
                      progressBar.value = loaded;
                    }
                  },
                );

                let text = document.createElement("div");
                text.classList.add("view", "text", "text--caption");
                xhr.addEventListener("load", () => {
                  let errorMessage = xhr.getResponseHeader(
                    "x-viewy-error-message",
                  );
                  console.log(errorMessage);
                  if (xhr.status >= 400 && xhr.status <= 599) {
                    text.style.color = "var(--color-destructive)";
                    if (errorMessage != null) {
                      text.textContent = errorMessage;
                    } else {
                      if (xhr.status === 413) {
                        text.textContent =
                          "Le fichier dépasse la taille maximale autorisé";
                      } else {
                        text.textContent = "Erreur lors de l'upload";
                      }
                    }

                    progressBar.replaceWith(text);
                    reject();
                  } else {
                    text.style.color = "var(--color-success)";
                    if (errorMessage != null) {
                      text.textContent = errorMessage;
                    } else {
                      text.textContent = "Fichier mis en ligne avec succès";
                    }
                    progressBar.replaceWith(text);
                    resolve();
                  }
                });
                xhr.addEventListener("error", () => {
                  text.textContent = "Erreur";
                  progressBar.replaceWith(text);
                  reject();
                });
                xhr.send(formData);
              }),
          ),
        );
        if (input.hasAttribute("data-redirect-uri")) {
          window.location.replace(input.getAttribute("data-redirect-uri"));
        }
      }

      let instance_id = fileInputComponent.getAttribute("data-file-list");
      let fileInput;
      if (
        fileInputComponent.classList.contains("multiple-file-input--hidden")
      ) {
        fileInput = fileInputComponent;
      } else {
        fileInput = fileInputComponent.querySelector("input[type='file']");
      }
      let file_list = document.getElementById(instance_id);
      file_list
        .querySelector(
          ".multiple-file-input--hidden__file-list__control-bar__close-button",
        )
        .addEventListener("click", (e) => {
          file_list.style.display = "none";
        });
      let file_list_content = file_list.querySelector(
        ".multiple-file-input__file-list__content",
      );
      let template = file_list.querySelector("template").content;
      fileInput.addEventListener("change", (e) => {
        file_list_content.innerHTML = "";
        file_list.style.display = "flex";
        let files = [...e.target.files].map((file) => [
          crypto.randomUUID(),
          file,
        ]);

        files.forEach(([id, file]) => {
          let template_instance = template.cloneNode(true);
          template_instance
            .querySelector(".multiple-file-input__file-list__item")
            .setAttribute("data-file-id", id);
          template_instance.querySelector(".t-file-name").textContent =
            file.name;
          file_list_content.appendChild(template_instance);
        });
        uploadFile(files);
      });
    });
});
