(map => {
  const mapUrl = document.currentScript.src;
  const resolve = imports => Object.fromEntries(Object.entries(imports ).map(([k, v]) => [k, new URL(v, mapUrl).href]));
  document.head.appendChild(Object.assign(document.createElement("script"), {
    type: "importmap",
    innerHTML: JSON.stringify({
      imports: resolve(map.imports),
      scopes: Object.fromEntries(Object.entries(map.scopes).map(([k, v]) => [new URL(k, mapUrl).href, resolve(v)]))
    })
  }));
})
({
  "imports": {
    "morphdom": "https://ga.jspm.io/npm:morphdom@2.7.7/dist/morphdom-esm.js",
    "floating-ui": "https://cdn.jsdelivr.net/npm/@floating-ui/dom@1.7.4/+esm"
  },
  "scopes": {
    "./": {
      "viewy": "./src/core.js",
      "viewy/widgets/": './src/widgets/',
    }
  }
});
