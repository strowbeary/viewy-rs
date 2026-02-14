import { startViewy } from "../core.js";

const COMPONENT_HOST_SELECTOR = '[data-v-component-host="true"]';
const COMPONENT_MESSAGE_SELECTOR = "[data-v-component-msg]";

function isValidElement(node) {
  return node && typeof node.closest === "function";
}

function readHostContext(host) {
  const eventUrl = host.getAttribute("data-v-component-event-url");
  const componentName = host.getAttribute("data-v-component-name");
  const componentId = host.getAttribute("data-v-component-id");
  const versionRaw = host.getAttribute("data-v-component-version") || "0";

  if (!eventUrl || !componentId || !componentName) {
    throw new Error("Missing required interactive component host attributes");
  }

  return {
    eventUrl,
    componentName,
    componentId,
    version: Number.parseInt(versionRaw, 10) || 0,
  };
}

function collectHostFields(host) {
  const form = new URLSearchParams();

  host
    .querySelectorAll("input[name], select[name], textarea[name]")
    .forEach((field) => {
      const name = field.getAttribute("name");
      if (!name) {
        return;
      }

      const tag = field.tagName.toLowerCase();
      const type = (field.getAttribute("type") || "").toLowerCase();

      if (
        tag === "input" &&
        (type === "checkbox" || type === "radio") &&
        !field.checked
      ) {
        return;
      }

      if (tag === "select" && field.multiple) {
        Array.from(field.selectedOptions).forEach((option) => {
          form.append(name, option.value);
        });
        return;
      }

      form.append(name, field.value || "");
    });

  return form;
}

async function dispatchHypermediaComponentMessage(host, rawMessage) {
  const context = readHostContext(host);
  const form = collectHostFields(host);
  form.set("_v_component_name", context.componentName);
  form.set("_v_component_id", context.componentId);
  form.set("_v_component_msg", rawMessage);
  form.set("_v_component_version", String(context.version));

  const response = await fetch(context.eventUrl, {
    method: "POST",
    headers: {
      "content-type": "application/x-www-form-urlencoded;charset=UTF-8",
    },
    body: form.toString(),
  });

  if (!response.ok) {
    throw new Error(`Component event failed with status ${response.status}`);
  }

  const html = await response.text();
  host.innerHTML = html;
  host.setAttribute("data-v-component-version", String(context.version + 1));
  startViewy(host);
}

function bindTrigger(trigger) {
  if (trigger.dataset.vComponentBound === "true") {
    return;
  }

  const rawMessage = trigger.getAttribute("data-v-component-msg");
  if (!rawMessage) {
    return;
  }

  const eventName = trigger.getAttribute("data-v-component-event") || "click";
  trigger.dataset.vComponentBound = "true";

  trigger.addEventListener(eventName, async (event) => {
    event.preventDefault();
    if (!isValidElement(trigger)) {
      return;
    }

    const host = trigger.closest(COMPONENT_HOST_SELECTOR);
    if (!host) {
      console.error(
        "Interactive component trigger is not inside a component host",
        trigger,
      );
      return;
    }

    try {
      await dispatchHypermediaComponentMessage(host, rawMessage);
    } catch (error) {
      console.error("Interactive component dispatch failed", error);
    }
  });
}

function bindHost(host) {
  host.querySelectorAll(COMPONENT_MESSAGE_SELECTOR).forEach(bindTrigger);
}

export function init(root) {
  if (!root) {
    return;
  }

  if (
    typeof root.matches === "function" &&
    root.matches(COMPONENT_HOST_SELECTOR)
  ) {
    bindHost(root);
  }

  if (typeof root.querySelectorAll === "function") {
    root.querySelectorAll(COMPONENT_HOST_SELECTOR).forEach(bindHost);
  }
}
