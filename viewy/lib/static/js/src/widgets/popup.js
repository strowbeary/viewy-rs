import morphdom from "morphdom";
import { load_injectable_content } from "../core.js";
import { actions as popover } from "viewy/widgets/popover.js";

export const actions = {
  async open_popup(popup_opener) {
    let popup = document.createElement("div");
    popup.classList.add("popup");

    popup.id = popup_opener.dataset.vTargetPopup;

    let popup_window = document.createElement("div");
    popup_window.classList.add("popup__window");

    let popup_window_content = document.createElement("div");
    popup_window_content.classList.add("popup__window__window-content");

    let popup_window_bar = document.createElement("div");
    popup_window_bar.classList.add("popup__window__window-bar");

    popup_window.append(popup_window_bar);
    popup_window.append(popup_window_content);
    popup.append(popup_window);

    document.body.append(popup);
    const content_url = popup_opener.dataset.vUrl;
    if (content_url) {
      await load_injectable_content(content_url, popup_window_content);
    }

    popover.close_all_popover();
    popup.classList.add("visible");

    popup.addEventListener("click", (e) => {
      if (e.target === popup) {
        popup.addEventListener("transitionend", () => {
          popup.remove();
        });
        popup.classList.remove("visible");
      }
    });
  },
};
