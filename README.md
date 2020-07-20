# neon-rodio

> Rodio for Node.js

## Installation

Before installation, see [Install Node Build Tools](https://neon-bindings.com/docs/getting-started#install-node-build-tools)

```bash
npm install neon-rodio
yarn add neon-rodio
```

## Usage

```javascript
const rodio = require("neon-rodio");

const player = new rodio();

player.load("path/to/test.wav");
```

## Methods

- load(url: string): boolean
- play(): boolean
- pause(): void
- stop(): void
- volume(): number
- setVolume(level: number): void
- isPaused(): boolean
- empty(): boolean
- position(): number
