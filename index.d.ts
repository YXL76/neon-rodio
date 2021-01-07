declare module "neon-rodio" {
  export default class NeonRodio {
    empty(url: string): boolean;
    load(): boolean;
    pause(): void;
    play(): boolean;
    volume(level: number): void;
    stop(): void;
  }
}
