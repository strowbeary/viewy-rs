function debounce(func, timeout = 300){
    let timer;
    return (...args) => {
        clearTimeout(timer);
        timer = setTimeout(() => { func.apply(this, args); }, timeout);
    };
}

/**
 *
 * @param {HTMLFormElement} form
 * @param {HTMLElement} root
 */
function asyncSubmit(root, form) {
    const formData = new FormData(form);
    form.dispatchEvent(new CustomEvent("asyncSubmit"));
    /*if (form.dataset.dynamicContentName) {
        let dynamicContent = document.querySelector(`.dynamic-content[data-dynamic-content-name = ${form.dataset.dynamicContentName}]`)
        dynamicContent.innerHTML = "";
        dynamicContent.dispatchEvent(new CustomEvent("dynamicContentErased"));
    }*/
    let req_options = {
        method: form.method,
    };
    if (form.method === "post") {
        req_options = {
            method: form.method,
            body: formData
        };
    }
    fetch(form.getAttribute("action"), req_options)
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

    detail.root.querySelectorAll(".card[data-submit-form]")  .forEach(card => {
        let formName= card.getAttribute("data-submit-form");
        let form = document.getElementById(formName);
        console.log(formName, form);
        card.addEventListener("click", e => {
            if (form.hasAttribute("data-async")) {
                asyncSubmit(detail.root, form)
            } else {
                form.submit();
            }
        })
    })


    detail.root.querySelectorAll("form")
        .forEach(form => {
            console.log(form);
            form.querySelectorAll(".field")
                .forEach(textfield => {
                    let input = textfield.querySelector("input");
                    if (input) {
                        input.addEventListener("invalid", e => {
                            e.preventDefault();
                            let old_helper_text = textfield.querySelector(".field__helper-text");
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
            form.querySelectorAll("[data-submit-on-keypress]")
                .forEach(input => {
                    console.log("Submit on keypress input", input);
                    let debounced_fn = debounce(() => {
                        if (form.hasAttribute("data-async")) {
                            asyncSubmit(detail.root, form);
                        } else {
                            form.submit();
                        }
                    }, 200);
                    input.addEventListener("input", debounced_fn);
                });
            form.querySelectorAll(".field__rich-text-area")
                .forEach(field => {
                    let input = field.querySelector(".field__input");
                    let editor = field.querySelector(".field__editor");
                    let toolbar = field.querySelector(".field__toolbar");

                    let quill = new Quill(`#${editor.id}`, {
                        modules: {
                            toolbar: {
                                container: `#${toolbar.id}`,
                                handlers: {
                                    link(value) {
                                        if (value) {
                                            const range = this.quill.getSelection();
                                            if (range == null || range.length === 0) return;
                                            let preview = this.quill.getText(range);
                                            if (
                                                /^\S+@\S+\.\S+$/.test(preview) &&
                                                preview.indexOf('mailto:') !== 0
                                            ) {
                                                preview = `mailto:${preview}`;
                                            }
                                            const { tooltip } = this.quill.theme;
                                            tooltip.edit('link', preview);
                                        } else {
                                            this.quill.format('link', false, Quill.sources.USER);
                                        }
                                    },
                                    image() {
                                        let fileInput = this.container.querySelector(
                                            'input.ql-image[type=file]',
                                        );
                                        if (fileInput == null) {
                                            fileInput = document.createElement('input');
                                            fileInput.setAttribute('type', 'file');
                                            fileInput.setAttribute(
                                                'accept',
                                                this.quill.uploader.options.mimetypes.join(', '),
                                            );
                                            fileInput.classList.add('ql-image');
                                            fileInput.addEventListener('change', () => {
                                                const range = this.quill.getSelection(true);
                                                this.quill.uploader.upload(range, fileInput.files);
                                                fileInput.value = '';
                                            });
                                            this.container.appendChild(fileInput);
                                        }
                                        fileInput.click();
                                    },
                                }
                            },
                        },
                        theme: null
                    });

                    console.log(quill);

                    quill.on("text-change", function () {
                        input.value = editor.querySelector(".ql-editor").innerHTML;
                    });
                })
        });
});