// 앱 부트스트랩. Tauri 예제와 동일한 도메인 추상화를 공유한다.

import { Rectangle } from "../shapes";
import { DomShapeRenderer, type ShapeRenderer } from "../render";

export class App {
  private readonly container: HTMLElement;
  private readonly renderer: ShapeRenderer;

  constructor(container: HTMLElement, renderer: ShapeRenderer = new DomShapeRenderer()) {
    this.container = container;
    this.renderer = renderer;
  }

  start(): void {
    const rectangle = new Rectangle({
      width: 200,
      height: 200,
      color: "tomato",
      borderRadius: 16,
    });
    this.renderer.render(rectangle, this.container);
  }
}
