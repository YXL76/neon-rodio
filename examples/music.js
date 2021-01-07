const path = require("path");
const NeonRodio = require("../../neon-rodio");

const player = new NeonRodio();

player.load(path.resolve(__dirname, "music.wav"));

let flag = false;

setInterval(() => {
  if (flag) {
    player.play();
  } else {
    player.pause();
  }
  flag = !flag;
}, 2000);
