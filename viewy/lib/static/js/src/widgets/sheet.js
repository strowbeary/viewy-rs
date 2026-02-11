import { load_injectable_content } from "../core.js";
import { actions as popover } from "viewy/widgets/popover.js";

function close_sheet(sheet) {
  sheet.addEventListener("transitionend", () => {
    sheet.remove();
  });
  sheet.classList.remove("visible");
  sheet.classList.remove("sheet--loading");
}

function build_sheet(edge) {
  let sheet = document.createElement("div");
  sheet.classList.add("sheet");
  sheet.classList.add("sheet--overlay");
  sheet.classList.add(`sheet--edge-${edge}`);

  let scrim = document.createElement("div");
  scrim.classList.add("sheet__scrim");

  let panel = document.createElement("div");
  panel.classList.add("sheet__panel");

  let loader = document.createElement("div");
  loader.classList.add("sheet__panel__loader");

  let spinner = document.createElement("div");
  spinner.classList.add("sheet__panel__spinner");
  loader.append(spinner);

  let content = document.createElement("div");
  content.classList.add("sheet__panel__content");

  panel.append(loader);
  panel.append(content);
  sheet.append(scrim);
  sheet.append(panel);

  scrim.addEventListener("click", () => close_sheet(sheet));
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && sheet.classList.contains("visible")) {
      close_sheet(sheet);
    }
  });

  return { sheet, content };
}

export const actions = {
  async open_sheet(sheet_opener) {
    document.querySelectorAll(".sheet").forEach((existing) => {
      existing.remove();
    });

    popover.close_all_popover();

    let edge = sheet_opener.dataset.vSheetEdge || "right";
    if (!["left", "right", "bottom"].includes(edge)) {
      edge = "right";
    }

    const { sheet, content } = build_sheet(edge);

    document.body.append(sheet);

    const content_url = sheet_opener.dataset.vUrl;
    sheet.classList.add("sheet--loading");
    sheet.getBoundingClientRect();
    requestAnimationFrame(() => {
      sheet.classList.add("visible");
    });

    if (content_url) {
      await load_injectable_content(content_url, content);
    }

    sheet.classList.remove("sheet--loading");
  },
};
