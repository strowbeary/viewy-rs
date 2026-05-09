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

function build_morph_target(injection_root, injectable_content) {
  const template = document.createElement("template");
  template.innerHTML = injectable_content.trim();

  const first_element = template.content.firstElementChild;
  const has_single_element =
    first_element &&
    template.content.childElementCount === 1 &&
    template.content.textContent.trim() === "";

  if (has_single_element) {
    return first_element;
  }

  const container = injection_root.cloneNode();
  container.innerHTML = injectable_content;
  return container;
}

export async function load_injectable_content(
  url,
  injection_root,
  request_options = {},
) {
  const headers = new Headers(request_options.headers || {});
  if (!headers.has("x-viewy-render-mode")) {
    headers.set("x-viewy-render-mode", "ContentOnly");
  }

  let res = await fetch(url, {
    ...request_options,
    headers,
  });
  if (!res.ok) {
    throw new Error(`Request failed with status ${res.status}`);
  }
  let injectable_content = await res.text();
  let morph_target = build_morph_target(injection_root, injectable_content);
  let result = morphdom(injection_root, morph_target, {
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

  if (
    root.querySelector(".nav") ||
    (typeof root.matches === "function" && root.matches(".nav"))
  ) {
    import("viewy/widgets/nav.js").then((nav) => {
      nav.init(root);
    });
  }
  initActions(root);
});
