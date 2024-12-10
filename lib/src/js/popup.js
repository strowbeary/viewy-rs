function closeAllPopover() {
  document.querySelectorAll(".popover").forEach((popover) => {
    popover.removeAttribute("data-show");
  });
}

window.addEventListener("startViewy", ({ detail }) => {
  const popups = detail.root.querySelectorAll(".popup");

  function closeAll(excludedPopupIds) {
    //closeAllPopover();
    popups.forEach((popup) => {
      let popupId = popup.getAttribute("data-attach-to");

      if (!excludedPopupIds.includes(popupId)) {
        popup.classList.remove("visible");
      }
    });
  }

  popups.forEach((popup) => {
    let open_on_startup = popup.classList.contains("visible");
    let attached_form = popup.getAttribute("data-form-to-submit-on-open");
    if (attached_form && open_on_startup) {
      console.log(attached_form);
      let form = document.getElementById(attached_form);
      if (form.hasAttribute("data-async")) {
        asyncSubmit(detail.root, form);
      } else {
        form.submit();
      }
    }
  });

  function open(popupId) {
    let popup = document.querySelector(`.popup[data-attach-to="${popupId}"]`);
    let opennedPopups = [...detail.root.querySelectorAll(`.popup.visible`)].map(
      (popup) => popup.getAttribute("data-attach-to"),
    );
    closeAll([popupId, ...opennedPopups]);
    window.dispatchEvent(new CustomEvent("viewy-popup-open"));
    popup.classList.add("visible");

    popup.querySelectorAll("iframe[data-old-src]").forEach((iframe) => {
      iframe.src = iframe.getAttribute("data-old-src");
      iframe.removeAttribute("data-old-src");
    });

    let attached_form = popup.getAttribute("data-form-to-submit-on-open");
    if (attached_form) {
      console.log(attached_form);
      let form = document.getElementById(attached_form);
      if (form.hasAttribute("data-async")) {
        asyncSubmit(detail.root, form);
      } else {
        form.submit();
      }
    }
  }

  function close(popupId) {
    let popup = document.querySelector(`.popup[data-attach-to="${popupId}"]`);
    popup.classList.remove("visible");

    popup.querySelectorAll("iframe").forEach((iframe) => {
      iframe.setAttribute("data-old-src", iframe.src);
      iframe.src = "";
    });
  }

  function init() {
    popups.forEach((popup) => {
      const popupId = popup.getAttribute("data-attach-to");
      function close_with_form_check(popupId) {
        close(popupId);
        let attached_form = popup.getAttribute("data-form-to-submit-on-close");
        if (attached_form) {
          console.log(attached_form);
          let form = document.getElementById(attached_form);
          if (form.hasAttribute("data-async")) {
            asyncSubmit(detail.root, form);
          } else {
            form.submit();
          }
        }
      }
      popup.addEventListener("click", (e) => {
        if (e.target === popup) {
          close_with_form_check(popupId);
        }
      });
      popup.querySelectorAll(".popup__window-controls").forEach((el) => {
        el.addEventListener("click", () => {
          close_with_form_check(popupId);
        });
      });

      const el = document.getElementById(popupId);
      el.addEventListener("click", (e) => {
        e.preventDefault();
        e.stopPropagation();
        open(popupId);
      });
      el.querySelectorAll("a, .button, .clickable, .file-input").forEach(
        (clickable) => {
          if (
            !clickable.classList.contains("popup--opener") ||
            !clickable.classList.contains("popover--opener")
          ) {
            clickable.addEventListener("click", (e) => {
              e.stopPropagation();
            });
          }
        },
      );
    });
  }

  init();
});
