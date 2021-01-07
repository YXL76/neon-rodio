const path = require("path");
const { playerNew, playerLoad, playerPosition } = require("../../neon-rodio");

const player = playerNew();

playerLoad(player, path.resolve(__dirname, "music.wav"));

setInterval(() => console.log(playerPosition(player)), 1000);
