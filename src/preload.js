const lib = require("../dist")

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

  console.log("hello")
  const hello = lib.hello();
  const helloElement = document.getElementById("hello");
  helloElement.innerText = hello;

  console.log("number-of-cpus")
  const numberOfCPUs = lib.num_cpus();
  const numberOfCPUsElement = document.getElementById("number-of-cpus");
  numberOfCPUsElement.innerText = numberOfCPUs;

  console.log("acbr-version")
  const acbrVersion = lib.get_version();
  const acbrVersionElement = document.getElementById("acbr-version");
  acbrVersionElement.innerText = acbrVersion;


});