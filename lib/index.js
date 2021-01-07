const {
  playerEmpty,
  playerLoad,
  playerNew,
  playerPause,
  playerPlay,
  playerVolume,
  playerStop,
} = require("../index.node");

class NeonRodio {
  constructor() {
    this.player = playerNew();
  }

  empty() {
    return playerEmpty(this.player);
  }

  load(url) {
    return playerLoad(this.player, url);
  }

  pause() {
    playerPause(this.player);
  }

  play() {
    return playerPlay(this.player);
  }

  volume(level) {
    return playerVolume(this.player, level);
  }

  stop() {
    playerStop(this.player);
  }
}

module.exports = NeonRodio;
