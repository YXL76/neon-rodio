declare module "neon-rodio" {
  export default class NeonRodio {
    empty(): boolean;
    load(url: string): boolean;
    pause(): void;
    play(): boolean;
    volume(level: number): void;
    stop(): void;
  }
}
