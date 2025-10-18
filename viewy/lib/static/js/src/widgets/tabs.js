import {load_injectable_content} from "../core.js";

export async function init(root) {
    let tabContainers = root.querySelectorAll('.tab-container');
    for (const tabContainer of tabContainers) {
        let tabs = tabContainer.querySelectorAll('.tab-container__button-container__tab');
        let tabContent = tabContainer.querySelector('.tab-container__tab-content');
        let activeTab = tabContainer.querySelector('.tab-container__button-container__tab.active');
        if (!activeTab) {
            tabs[0].classList.add('active');
            activeTab = tabs[0];
        }

        await load_injectable_content(
            activeTab.dataset.vUrl,
            tabContent
        );

        for (let tab of tabs) {
            tab.addEventListener("click", async () => {
                tabs.forEach((tab) => {
                    tab.classList.remove('active');
                });
                tab.classList.add('active');
                await load_injectable_content(
                    tab.dataset.vUrl,
                    tabContent
                );
            })
        }
    }
}