function debounce(func, timeout = 300) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);
    }, timeout);
  };
}

/**
 *
 * @param {HTMLFormElement} form
 * @param {HTMLElement} root
 */
function asyncSubmit(root, form, submitter) {
  const formData = new FormData(form);
  if (submitter && submitter.getAttribute("name")) {
    formData.set(
      submitter.getAttribute("name"),
      submitter.getAttribute("value"),
    );
  }

  form.dispatchEvent(new CustomEvent("asyncSubmit"));
  /*if (form.dataset.dynamicContentName) {
        let dynamicContent = document.querySelector(`.dynamic-content[data-dynamic-content-name = ${form.dataset.dynamicContentName}]`)
        dynamicContent.innerHTML = "";
        dynamicContent.dispatchEvent(new CustomEvent("dynamicContentErased"));
    }*/
  let req_options = {
    method: form.method,
    redirect: "follow",
  };
  if (form.method === "post") {
    req_options = {
      body: formData,
      ...req_options,
    };
  }
  fetch(form.getAttribute("action"), req_options)
    .then((response) => {
      if (response.redirected) {
        window.location.href = response.url;
      } else {
        return response;
      }
    })
    .then((res) => res.text())
    .then((content) => {
      if (form.dataset.dynamicContentName) {
        let dynamicContent = document.querySelector(
          `.dynamic-content[data-dynamic-content-name = ${form.dataset.dynamicContentName}]`,
        );

        dynamicContent.innerHTML = content;

        window.dispatchEvent(
          new CustomEvent("startViewy", {
            detail: {
              root: dynamicContent,
            },
          }),
        );
        let popups = dynamicContent.querySelectorAll(".popup");
        document.body.append(...popups);
        let popovers = dynamicContent.querySelectorAll(".popover");
        document.body.append(...popovers);
      }
    });
}

window.addEventListener("startViewy", ({ detail }) => {
  detail.root.querySelectorAll("form[data-async]").forEach((form) => {
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      console.log(e);
      asyncSubmit(detail.root, form, e.submitter);
    });
  });

  detail.root.querySelectorAll(".card[data-submit-form]").forEach((card) => {
    let formName = card.getAttribute("data-submit-form");
    let form = document.getElementById(formName);
    console.log(formName, form);
    card.addEventListener("click", (e) => {
      if (form.hasAttribute("data-async")) {
        asyncSubmit(detail.root, form, null);
      } else {
        form.submit();
      }
    });
  });

  detail.root.querySelectorAll("form").forEach((form) => {
    console.log(form);
    form.querySelectorAll(".field").forEach((textfield) => {
      let input = textfield.querySelector("input");
      if (input) {
        input.addEventListener("invalid", (e) => {
          e.preventDefault();
          let old_helper_text = textfield.querySelector(".field__helper-text");
          if (old_helper_text !== null) {
            old_helper_text.remove();
          }
          let helper_text = document.createElement("div");
          helper_text.classList.add(
            "view",
            "text",
            "text--caption",
            "textfield__helper-text",
          );
          helper_text.textContent = "Ce champs doit être rempli";
          textfield.appendChild(helper_text);
          textfield.classList.add("textfield--error");
          input.addEventListener(
            "input",
            () => {
              textfield.classList.remove("textfield--error");
              if (old_helper_text !== null) {
                helper_text.replaceWith(old_helper_text);
              } else {
                helper_text.remove();
              }
            },
            { once: true },
          );
        });
      }
    });
    form.querySelectorAll("[data-auto-submit]").forEach((input) => {
      console.log("Auto submit input", input);
      input.addEventListener("change", () => {
        if (form.hasAttribute("data-async")) {
          asyncSubmit(detail.root, form, null);
        } else {
          form.submit();
        }
      });
    });
    form.querySelectorAll("[data-submit-on-keypress]").forEach((input) => {
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
    form.querySelectorAll(".field__rich-text-area").forEach((field) => {
      let input = field.querySelector(".field__input");
      let editor = field.querySelector(".field__editor");
      let toolbar = field.querySelector(".field__toolbar");
      const Link = Quill.import("formats/link");
      console.log("Link", Link);
      Quill.register("modules/blotFormatter", QuillBlotFormatter.default);
      const toolbarOptions = [
        [{ header: [1, 2, 3, 4, 5, 6, false] }],
        ["bold", "italic", "underline"],
        ["link", "image"],
        [{ list: "ordered" }, { list: "bullet" }, { list: "check" }],
        ["clean"],
      ];
      let quill = new Quill(`#${editor.id}`, {
        theme: "snow",
        bounds: editor,
        modules: {
          toolbar: toolbarOptions,
          blotFormatter: {},
        },
      });

      console.log(quill);

      quill.on("text-change", function () {
        input.value = editor.querySelector(".ql-editor").innerHTML;
      });
    });
  });
});
