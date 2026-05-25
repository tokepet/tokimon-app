// 도메인 추상화. 어떤 OS/프레임워크인지 모른다.
// Circle, Polygon 등 새 도형을 추가하려면 이 클래스를 상속한다.

export interface ShapeStyle {
  width: number;
  height: number;
  color: string;
  borderRadius?: number;
}

export abstract class Shape {
  protected style: ShapeStyle;

  constructor(style: ShapeStyle) {
    this.style = style;
  }

  getStyle(): Readonly<ShapeStyle> {
    return this.style;
  }

  setColor(color: string): void {
    this.style = { ...this.style, color };
  }

  // 각 도형이 자기 자신을 CSS로 어떻게 표현하는지를 안다.
  // 렌더링 백엔드(DOM/Canvas/SVG)와는 무관한 순수 CSS 속성을 반환.
  abstract toCSSStyle(): Partial<CSSStyleDeclaration>;
}
