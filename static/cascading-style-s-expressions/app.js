import init, { string_to_stylesheet } from "./pkg/css.js";

const input = document.getElementById("input");
const output = document.getElementById("output");
const sheet = document.createElement("style");
document.head.appendChild(sheet);

input.value = `(textarea width 100% height 30vh)
(article border (1px solid black) (p color red))
(.textarea-label font-weight bold)`;

function setOutput(src) {
  const css = string_to_stylesheet(src);
  output.textContent = css;
  sheet.innerHTML = css;
}

init().then(() => {
  setOutput(input.value);
  input.addEventListener("input", (e) => setOutput(e.target.value));
});
