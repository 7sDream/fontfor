const colToHex = (c) => {
  let color = (c < 75) ? c + 75 : c
  let hex = color.toString(16);
  return hex.length == 1 ? "0" + hex : hex;
}

const rgbToHex = (r,g,b) => {
  return "#" + colToHex(r) + colToHex(g) + colToHex(b);
}

const getRandomColor = () => {
  return rgbToHex(
    Math.floor(Math.random() * 255),
    Math.floor(Math.random() * 255),
    Math.floor(Math.random() * 255));
}

document.addEventListener("DOMContentLoaded", function () {
  const cards = document.getElementsByClassName("card");
  for (i = 0; i < cards.length; i++) {
    cards[i].style.backgroundColor = getRandomColor();
  }
});
