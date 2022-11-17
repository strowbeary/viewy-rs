window.addEventListener("startViewy", ({detail}) => {
    detail.root.querySelectorAll(".field--duration")
        .forEach((field) => {
           const start_date_input = field.querySelector("input[name$='_start_datetime']");
           const end_date_input = field.querySelector("input[name$='_end_datetime']");
            field.querySelectorAll(".tag[data-duration]")
                .forEach(tag => {
                    tag.addEventListener("click", ({target}) => {
                        const duration = parseInt(tag.dataset["duration"]);
                        let start_date = new Date(start_date_input.value + 'Z');
                        start_date = new Date(start_date.getTime() + duration*60000);
                        let splittedDate = start_date.toISOString().split("T");
                        end_date_input.value = `${splittedDate[0]}T${splittedDate[1].slice(0, 5)}`;
                    })
                })
        });

});