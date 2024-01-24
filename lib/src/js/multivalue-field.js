
window.addEventListener("startViewy", ({detail}) => {
    if ("content" in document.createElement("template")) {
        document.querySelectorAll(".field--multi-value").forEach(field => {

            field.querySelectorAll(".field--multi-value__value-list__value")
                .forEach((value) => value.querySelector("button").addEventListener("click", () => value.remove()))


            const template = field.querySelector("#value-template");
            const valueList = field.querySelector(".field--multi-value__value-list");
            field.querySelector(".field--multi-value__add-value-button")
                .addEventListener("click", () => {
                    const clone = document.importNode(template.content, true);

                    let valueRow = clone.querySelector(".field--multi-value__value-list__value");
                    valueRow.querySelector("button").addEventListener("click", () => valueRow.remove())
                    valueList.appendChild(clone);
                })
        })
    }
});
