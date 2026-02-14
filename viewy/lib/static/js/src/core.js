import morphdom from "morphdom";
import { init as initActions } from "./actions.js";

const _addEventListener = EventTarget.prototype.addEventListener;

EventTarget.prototype.addEventListener = function (type, listener, options) {
  this.__hasListeners = true;
  return _addEventListener.call(this, type, listener, options);
};

export function startViewy(root) {
  window.dispatchEvent(
    new CustomEvent("startViewy", {
      detail: {
        root,
      },
    }),
  );
}

export async function load_injectable_content(url, injection_root) {
  let res = await fetch(url, {
    headers: {
      "x-viewy-render-mode": "ContentOnly",
    },
  });
  let injectable_content = await res.text();
  let old_class_list = injection_root.classList;
  let old_dataset = injection_root.dataset;
  //injection_root.insertAdjacentHTML("beforeend", injectable_content);
  let container = injection_root.cloneNode();
  container.innerHTML = injectable_content;
  let result = morphdom(injection_root, container, {
    onElUpdated(el) {
      if (el.__hasListeners) {
        el.replaceWith(el.cloneNode(true));
      }
    },
    onBeforeElUpdated: function (fromEl, toEl) {
      // spec - https://dom.spec.whatwg.org/#concept-node-equals
      if (fromEl.isEqualNode(toEl)) {
        return false;
      }

      return true;
    },
  });
  startViewy(result);
}

window.addEventListener("startViewy", (event) => {
  let root = event.detail.root;
  console.log("Viewy started", root);

  if (root.querySelector(".button")) {
    import("viewy/widgets/form.js").then((form) => {
      form.init(root);
    });
  }

  if (root.querySelector(".tab-container")) {
    import("viewy/widgets/tabs.js").then((tabs) => {
      tabs.init(root);
    });
  }

  if (
    root.querySelector(".select, .picker") ||
    (typeof root.matches === "function" && root.matches(".select, .picker"))
  ) {
    import("viewy/widgets/picker.js").then((picker) => {
      picker.init(root);
    });
  }

  if (
    root.querySelector('[data-v-component-host="true"]') ||
    (typeof root.matches === "function" &&
      root.matches('[data-v-component-host="true"]'))
  ) {
    import("viewy/widgets/interactive_component.js").then((interactive) => {
      interactive.init(root);
    });
  }
  initActions(root);
});
