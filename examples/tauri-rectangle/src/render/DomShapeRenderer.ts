import { Shape } from "../shapes";
import { ShapeRenderer } from "./ShapeRenderer";

export class DomShapeRenderer implements ShapeRenderer {
  render(shape: Shape, container: HTMLElement): void {
    this.clear(container);
    const el = document.createElement("div");
    Object.assign(el.style, shape.toCSSStyle());
    container.appendChild(el);
  }

  clear(container: HTMLElement): void {
    container.innerHTML = "";
  }
}
