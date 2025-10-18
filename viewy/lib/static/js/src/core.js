import morphdom from 'morphdom';
import {init as initActions} from './actions.js';

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
            "x-viewy-render-mode": "ContentOnly"
        }
    });
    let injectable_content = await res.text();
    let old_class_list = injection_root.classList;
    let old_dataset = injection_root.dataset;
    //injection_root.insertAdjacentHTML("beforeend", injectable_content);
    let container = injection_root.cloneNode();
    container.innerHTML = injectable_content;
    let result = morphdom(injection_root, container);
    startViewy(result);
}


window.addEventListener("startViewy", (event) => {
    let root = event.detail.root;
    console.log("Viewy started", root);

    if (root.querySelector(".button")) {
        import("viewy/widgets/form.js").then(form => {
            form.init(root);
        });
    }

    if (root.querySelector(".tab-container")) {
        import("viewy/widgets/tabs.js").then(tabs => {
            tabs.init(root);
        });
    }
    initActions(root)

});
