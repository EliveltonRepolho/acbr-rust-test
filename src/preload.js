const lib = require("../index.node");
// All of the Node.js APIs are available in the preload process.
// It has the same sandbox as a Chrome extension.
window.addEventListener("DOMContentLoaded", () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector);
    if (element) element.innerText = text;
  };

  for (const dependency of ["chrome", "node", "electron"]) {
    replaceText(`${dependency}-version`, process.versions[dependency]);
  }
  const numberOfCPUs = lib.num_cpus();
  const numberOfCPUsElement = document.getElementById("number-of-cpus");
  numberOfCPUsElement.innerText = numberOfCPUs;

  const acbrCersion = lib.version();
  const acbrCersionElement = document.getElementById("acbr-version");
  acbrCersionElement.innerText = acbrCersion;
});