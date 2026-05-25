// 도형을 어디에 어떻게 그릴지에 대한 추상화.
// DOM 외에 Canvas/SVG/WebGL 등 다른 백엔드를 추가하려면 이 인터페이스를 구현한다.

import { Shape } from "../shapes";

export interface ShapeRenderer {
  render(shape: Shape, container: HTMLElement): void;
  clear(container: HTMLElement): void;
}
