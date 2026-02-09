import { getVOnEventNames, querySelectorByAttrPrefix } from "./helpers.js";
import { actions } from "./widgets/popup.js";

export function init(root) {
  for (const el of querySelectorByAttrPrefix("data-v-on-", root)) {
    for (const eventName of getVOnEventNames(el)) {
      let action = el.getAttribute(`data-v-on-${eventName}`);
      switch (action) {
        case "open_popup":
        case "close_parent_popup":
          import("viewy/widgets/popup.js").then((popup) => {
            el.addEventListener(eventName, () => popup.actions[action](el));
          });
          break;
        case "open_popover":
        case "close_parent_popover":
          import("viewy/widgets/popover.js").then((popover) => {
            el.addEventListener(eventName, () => popover.actions[action](el));
          });
          break;
        case "close_parent_window":
          el.addEventListener(eventName, () => {
            const popup = el.closest(".popup");
            const popover = el.closest(".popover");
            if (popup) {
              popup.addEventListener("transitionend", () => {
                popup.remove();
              });
              popup.classList.remove("visible");
            }
            if (popover) {
              popover.addEventListener("transitionend", () => {
                popover.remove();
              });
              popover.classList.remove("visible");
            }
          });

          break;
      }
    }
  }
}
