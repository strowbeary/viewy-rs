import {getVOnEventNames , querySelectorByAttrPrefix} from './helpers.js';
import {actions} from "./widgets/popup.js";

export function init(root) {
    for(const el of querySelectorByAttrPrefix("data-v-on-", root)) {
        for(const eventName of getVOnEventNames(el)) {
            console.log(el, eventName);
            let action = el.getAttribute(`data-v-on-${eventName}`);
            switch (action) {
                case "open_popup":
                    import("viewy/widgets/popup.js").then(popup => {
                      el.addEventListener(eventName, () => popup.actions[action](el));
                    });
                    break;
            }
        }
    }
}