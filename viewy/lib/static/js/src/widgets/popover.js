import { computePosition, flip, shift, offset, autoUpdate } from "floating-ui";

import { load_injectable_content } from "viewy";

document.addEventListener("click", (e) => {
  document.querySelectorAll(".popover").forEach((popover) => {
    let popover_opener = document.querySelector(
      `[data-v-target-popover="${popover.id}"]`,
    );
    if (
      e.target !== popover &&
      !popover.contains(e.target) &&
      e.target !== popover_opener &&
      !popover_opener.contains(e.target)
    ) {
      popover.remove();
    }
  });
});

export const actions = {
  close_all_popover() {
    document.querySelectorAll(".popover").forEach((popover) => {
      popover.remove();
    });
  },
  async open_popover(popover_opener) {
    const existing_popover = document.getElementById(
      popover_opener.dataset.vTargetPopover,
    );
    if (existing_popover) {
      existing_popover.remove();
    } else {
      let popover = document.createElement("div");
      popover.classList.add("popover");

      popover.id = popover_opener.dataset.vTargetPopover;

      document.body.append(popover);

      const content_url = popover_opener.dataset.vUrl;
      if (content_url) {
        await load_injectable_content(content_url, popover);
      }
      popover.classList.add("visible");

      autoUpdate(popover_opener, popover, () => {
        computePosition(popover_opener, popover, {
          placement: "bottom",
          middleware: [shift({ padding: 5 }), offset(6), flip()],
        }).then(({ x, y }) => {
          Object.assign(popover.style, {
            left: `${x}px`,
            top: `${y}px`,
          });
        });
      });
    }
  },
};
