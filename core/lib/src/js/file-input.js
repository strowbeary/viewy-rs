window.addEventListener("load", () => {
    document.querySelectorAll("[data-input-file]")
        .forEach(fileInputOpener => {
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
        });
});