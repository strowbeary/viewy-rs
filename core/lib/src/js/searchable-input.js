function throttle(func, wait, leading, trailing, context) {
    var ctx, args, result;
    var timeout = null;
    var previous = 0;
    var later = function () {
        previous = new Date;
        timeout = null;
        result = func.apply(ctx, args);
    };
    return function () {
        var now = new Date;
        if (!previous && !leading) previous = now;
        var remaining = wait - (now - previous);
        ctx = context || this;
        args = arguments;
        // Si la période d'attente est écoulée
        if (remaining <= 0) {
            // Réinitialiser les compteurs
            clearTimeout(timeout);
            timeout = null;
            // Enregistrer le moment du dernier appel
            previous = now;
            // Appeler la fonction
            result = func.apply(ctx, args);
        } else if (!timeout && trailing) {
            // Sinon on s’endort pendant le temps restant
            timeout = setTimeout(later, remaining);
        }
        return result;
    };
};

window.addEventListener("load", () => {
    document.querySelectorAll("[data-async-datalist]")
        .forEach(inputContainer => {
            let url = inputContainer.getAttribute("data-async-datalist");
            let datalist = document.createElement("datalist");
            let input = inputContainer.querySelector("input");
            input.setAttribute("autocomplete", "off");
            datalist.setAttribute("id", input.getAttribute("list"));
            inputContainer.appendChild(datalist);

            input.addEventListener("input", throttle(async e => {
                console.log(e.key, e.code);
                if (!e.isComposing && !["ArrowDown", "ArrowUp", "Enter"].includes(e.code)) {
                    const response = await fetch(`${url}${e.target.value}`);
                    const suggestions = await response.json();
                    datalist.innerText = "";
                    suggestions.Search.forEach(suggestion => {
                        let option = document.createElement("option");
                        option.setAttribute("value", suggestion.imdbID);
                        option.text = suggestion.Title;
                        datalist.appendChild(option);
                    })
                }
            }, 300, false, true));
        });
});