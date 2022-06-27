window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".sortable-stack")
        .forEach((stack) => {
            if (!stack.classList.contains("sortable-stack--disabled")) {
                Sortable.create(stack, {
                    handle: ".sortable-stack__item__handle",
                    sort: true,
                    animation: 150,  // ms, animation speed moving items when sorting, `0` — without animation
                    easing: "cubic-bezier(1, 0, 0, 1)",
                    onUpdate(e) {
                        const form = document.createElement("form");
                        form.method = "POST";
                        if (stack.dataset.action) {
                            form.action = stack.dataset.action;
                        }
                        const oldIndexInput = document.createElement("input");
                        oldIndexInput.name = "old_index";
                        oldIndexInput.value = e.oldIndex + 1;
                        const newIndexInput = document.createElement("input");
                        newIndexInput.name = "new_index";
                        newIndexInput.value = e.newIndex + 1;
                        form.append(oldIndexInput);
                        form.append(newIndexInput);
                        asyncSubmit(detail.root, form);
                    }
                });
            }
        });
});