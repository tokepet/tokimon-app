// 앱 부트스트랩. 도메인(Shape)과 렌더러(ShapeRenderer)를 조립해서 시작 신호만 보낸다.
// 의존성 주입 형태로 받기 때문에 테스트 시 가짜 렌더러로 교체 가능.

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
