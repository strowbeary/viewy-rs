window.addEventListener("load", () => {
    let pickers = document.querySelectorAll(".picker--segmented");
    for (const picker of pickers) {
        console.log(picker);
        let options = picker.querySelectorAll(".item");
        for (const option of options) {
            option.addEventListener("click", e => {
                console.log("CLICK");
                for (const option of options) {
                    option.classList.remove("selected");
                }
                option.classList.add("selected");
            })
        }
    }
});