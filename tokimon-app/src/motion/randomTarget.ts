export const WINDOW_W = 240;
export const WINDOW_H = 120;
export const SHAPE_PX = 80;

const WANDER_MARGIN_X = 0;
const WANDER_MARGIN_Y = 20;

export type Point = { x: number; y: number };

export function nextTarget(): Point {
  const minX = WANDER_MARGIN_X;
  const maxX = WINDOW_W - SHAPE_PX - WANDER_MARGIN_X;
  const minY = WANDER_MARGIN_Y;
  const maxY = WINDOW_H - SHAPE_PX - WANDER_MARGIN_Y;

  return {
    x: minX + Math.random() * Math.max(0, maxX - minX),
    y: minY + Math.random() * Math.max(0, maxY - minY),
  };
}
