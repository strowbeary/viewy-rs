window.addEventListener("load", () => {
    document.querySelectorAll("[data-input-file]")
        .forEach(fileInputOpener => {
            fileInputOpener.addEventListener("click", (e) => {
                e.preventDefault();
                let fileInput = document.getElementById(fileInputOpener.getAttribute("data-input-file"));

                fileInput.addEventListener("change", () => {
                    console.log("change");
                    fileInput.closest("form").submit();
                });
                fileInput.click();
            });
        });
});