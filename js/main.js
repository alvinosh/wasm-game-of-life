import { Universe } from "../pkg/index";

const cols = Math.floor(window.innerWidth / 16);
const rows = Math.floor(window.innerHeight / 16);

console.log(cols, rows);

const universe = Universe.new(cols, rows);

let lastx = -1;
let lasty = -1;
let dragging = false;
document.addEventListener("keydown", (event) => {
  if (event.key == " ") universe.toggle_pause();
  if (event.key.toLowerCase() == "w") universe.tick_once();
});

document.addEventListener("mousedown", (event) => {
  dragging = true;
  let x = Math.floor(event.clientX / 16);
  let y = Math.floor(event.clientY / 16);
  if (lastx == x && lasty == y) return;
  lastx = x;
  lasty = y;
  universe.toggle_cell(x, y);
});

document.addEventListener("mousemove", () => {
  if (!dragging) return;
  let x = Math.floor(event.clientX / 16);
  let y = Math.floor(event.clientY / 16);
  if (lastx == x && lasty == y) return;
  lastx = x;
  lasty = y;
  universe.toggle_cell(x, y);
});

document.addEventListener("mouseup", (event) => {
  dragging = false;
  let x = Math.floor(event.clientX / 16);
  let y = Math.floor(event.clientY / 16);
  if (lastx == x && lasty == y) return;
  lastx = x;
  lasty = y;
  universe.toggle_cell(x, y);
});

const render = () => {
  universe.tick();
  universe.render();
  requestAnimationFrame(render);
};

requestAnimationFrame(render);
