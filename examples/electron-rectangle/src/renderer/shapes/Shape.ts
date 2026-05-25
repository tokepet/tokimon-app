// 도메인 추상화. Electron이든 Tauri든 어떤 프레임워크인지 모른다.

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

  abstract toCSSStyle(): Partial<CSSStyleDeclaration>;
}
