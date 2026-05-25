export const WINDOW_W = 480;
export const WINDOW_H = 360;
export const SHAPE_PX = 80;

const WANDER_MARGIN_X = 134;
const WANDER_MARGIN_Y = 100;

export type Point = { x: number; y: number };

export function nextTarget(): Point {
  const minX = WANDER_MARGIN_X;
  const maxX = WINDOW_W - SHAPE_PX - WANDER_MARGIN_X;
  const minY = WANDER_MARGIN_Y;
  const maxY = WINDOW_H - SHAPE_PX - WANDER_MARGIN_Y;
  return {
    x: minX + Math.random() * (maxX - minX),
    y: minY + Math.random() * (maxY - minY),
  };
}
