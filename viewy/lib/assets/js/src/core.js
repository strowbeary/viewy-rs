
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
    let injectable_content = await res.json();
    injection_root.insertAdjacentHTML("beforeend", injectable_content.content);
    startViewy(injection_root);

    document.body.insertAdjacentHTML("beforeend", injectable_content.root_nodes);

    //startViewy(template.content);
}


window.addEventListener("startViewy", (event) => {
    let root = event.detail.root;
    console.log("Viewy started", root);

    if (root.querySelector(".button")) {
        import("viewy/widgets/form.js").then(form => {
            form.init(root);
        });
    }
    initActions(root)

});
