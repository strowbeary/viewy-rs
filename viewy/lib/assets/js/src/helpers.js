export function querySelectorByAttrPrefix(prefix, root = document) {
    return Array.from(root.querySelectorAll('*')).filter(el =>
        Array.from(el.attributes).some(attr => attr.name.startsWith(prefix))
    );
}

export function getVOnEventNames(el) {
    return Array.from(el.attributes).filter(attr => attr.name.startsWith('data-v-on-')).map(attr => attr.name.replace('data-v-on-', ''))
}