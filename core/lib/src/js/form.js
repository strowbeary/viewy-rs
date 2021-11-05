function asyncSubmit(form) {
    const formData = new FormData(form);
    fetch(form.getAttribute("action"), {
        method: "POST",
        body: formData
    })
        .then();
}

window.addEventListener("load", () => {
    document.querySelectorAll("form[data-async]")
        .forEach(form => {
            form.addEventListener("submit", e => {
                e.preventDefault();
                asyncSubmit(form)
            });
        });
    document.querySelectorAll("form")
        .forEach(form => {
            form.querySelectorAll(".textfield")
                .forEach(textfield => {
                    let input = textfield.querySelector("input");
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
                });
            form.querySelectorAll("[data-auto-submit]")
                .forEach(input => {
                    console.log("Auto submit input", input);
                    input.addEventListener("change", () => {
                        if (form.hasAttribute("data-async")) {
                            asyncSubmit(form)
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
                        console.log("text-change", editor.querySelector(".ql-editor").innerHTML);
                        input.value = editor.querySelector(".ql-editor").innerHTML;
                    });
                })
        });
});