# neon-rodio

> Rodio for Node.js

`neon-rodio` allows you to play music files using javascript

## Installation

```bash
npm install neon-rodio
yarn add neon-rodio
```

## Usage

```javascript
const NeonRodio = require("../../neon-rodio");

const player = new NeonRodio();

player.load("path/to/test.wav");
```

## APIs

```typescript
.empty(url: string): boolean;     // check if playback is empty
.load(): boolean;                 // load music file (return true if loading succeeded)
.pause(): void;                   // pause playback
.play(): boolean;                 // resume playback (return true if playback doesn't end)
.volume(level: number): void;     // set player volume (0-100)
.stop(): void;                    // stop playback
```
