import { Shape } from "../shapes";

export interface ShapeRenderer {
  render(shape: Shape, container: HTMLElement): void;
  clear(container: HTMLElement): void;
}
