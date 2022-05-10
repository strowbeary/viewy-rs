/**
 *
 * @param {HTMLFormElement} form
 * @param {HTMLElement} root
 */
function asyncSubmit(root, form) {
    const formData = new FormData(form);
    if (form.dataset.dynamicContentName) {
        let dynamicContent = document.querySelector(`.dynamic-content[data-dynamic-content-name = ${form.dataset.dynamicContentName}]`)
        dynamicContent.innerHTML = "";
        dynamicContent.dispatchEvent(new CustomEvent("dynamicContentErased"));
    }
    fetch(form.getAttribute("action"), {
        method: "POST",
        body: formData
    })
        .then(res => res.text())
        .then(content => {
            if (form.dataset.dynamicContentName) {
                let dynamicContent = document.querySelector(`.dynamic-content[data-dynamic-content-name = ${form.dataset.dynamicContentName}]`)
                dynamicContent.innerHTML = content;
                window.dispatchEvent(new CustomEvent("startViewy", {
                    detail: {
                        root: dynamicContent
                    }
                }));
            }
        });
}

window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll("form[data-async]")
        .forEach(form => {
            form.addEventListener("submit", e => {
                e.preventDefault();
                asyncSubmit(detail.root, form)
            });
        });
    detail.root.querySelectorAll("form")
        .forEach(form => {
            form.querySelectorAll(".textfield")
                .forEach(textfield => {
                    let input = textfield.querySelector("input");
                    if (input) {
                        input.addEventListener("invalid", e => {
                            e.preventDefault();
                            let old_helper_text = textfield.querySelector(".textfield__helper-text");
                            if (old_helper_text !== null) {
                                old_helper_text.remove();
                            }
                            let helper_text = document.createElement("div");
                            helper_text.classList.add("view", "text", "text--caption", "textfield__helper-text");
                            helper_text.textContent = "Ce champs doit être rempli";
                            textfield.appendChild(helper_text);
                            textfield.classList.add("textfield--error");
                            input.addEventListener("input", () => {
                                textfield.classList.remove("textfield--error");
                                if (old_helper_text !== null) {
                                    helper_text.replaceWith(old_helper_text);
                                } else {
                                    helper_text.remove();
                                }
                            }, {once: true});
                        });
                    }
                });
            form.querySelectorAll("[data-auto-submit]")
                .forEach(input => {
                    console.log("Auto submit input", input);
                    input.addEventListener("change", () => {
                        if (form.hasAttribute("data-async")) {
                            asyncSubmit(detail.root, form)
                        } else {
                            form.submit();
                        }
                    })
                });
            form.querySelectorAll(".textfield__rich-text-area")
                .forEach(field => {
                    let input = field.querySelector(".textfield__input");
                    let editor = field.querySelector(".textfield__editor");
                    let toolbar = field.querySelector(".textfield__toolbar");

                    let quill = new Quill(`#${editor.id}`, {
                        modules: {
                            toolbar: `#${toolbar.id}`
                        }
                    });

                    quill.on('text-change', () => {
                        console.log("text-input", input);
                        input.value = editor.querySelector(".ql-editor").innerHTML;
                    });
                })
        });
});