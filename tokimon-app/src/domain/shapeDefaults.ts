import type { ShapeKind } from "./ShapeKind";

const SHAPE_COLORS: Record<ShapeKind, string> = {
  triangle: "#ff6b6b",
  square: "#4dabf7",
  circle: "#51cf66",
};

export function defaultColor(kind: ShapeKind): string {
  return SHAPE_COLORS[kind];
}
