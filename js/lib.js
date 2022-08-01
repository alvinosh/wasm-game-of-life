const canvas = document.getElementById("canvas");
const size = 16;
const context = canvas.getContext("2d");
fitToContainer(canvas);

function fitToContainer(canvas) {
  // Make it visually fill the positioned parent
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  // ...then set the internal size to match
  canvas.width = canvas.offsetWidth;
  canvas.height = canvas.offsetHeight;
}

export function clear_screen() {
  context.clearRect(0, 0, canvas.width, canvas.height);
}

export function set_fill(color) {
  context.fillStyle = color;
}

export function draw_rect(x, y) {
  context.fillRect(x * size, y * size, size, size);
}

export function draw_screen_outline(color) {
  let size = 8;
  context.fillStyle = "#00000000";
  context.lineWidth = size;
  context.strokeStyle = color;
  context.beginPath();
  context.rect(size / 2, size / 2, canvas.width - size, canvas.height - size);
  context.stroke();
  context.fill();
}
