import { Shape, ShapeStyle } from "./Shape";

export class Rectangle extends Shape {
  constructor(style: ShapeStyle) {
    super(style);
  }

  toCSSStyle(): Partial<CSSStyleDeclaration> {
    return {
      width: `${this.style.width}px`,
      height: `${this.style.height}px`,
      backgroundColor: this.style.color,
      borderRadius: this.style.borderRadius
        ? `${this.style.borderRadius}px`
        : "0",
    };
  }
}
