function asyncSubmit(form) {
    console.log("Async form submit", form);
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
            console.log("Async form", form);
            form.addEventListener("submit", e => {
                e.preventDefault();
                asyncSubmit(form)
            });
        });
    document.querySelectorAll("form")
        .forEach(form => {
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
        });
});