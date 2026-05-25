import { ShapeView } from "./ShapeView";
import type { ShapeKind } from "../domain/ShapeKind";
import { defaultColor } from "../domain/shapeDefaults";
import {
  SHAPE_PX,
  WINDOW_H,
  WINDOW_W,
  type Point,
} from "../motion/randomTarget";

type Props = {
  onPick: (kind: ShapeKind, startAt: Point) => void;
};

const SLOT_W = WINDOW_W / 3;
const SHAPE_Y = (WINDOW_H - SHAPE_PX) / 2;
const SLOTS: { kind: ShapeKind; x: number }[] = [
  { kind: "triangle", x: 0 * SLOT_W + (SLOT_W - SHAPE_PX) / 2 },
  { kind: "square", x: 1 * SLOT_W + (SLOT_W - SHAPE_PX) / 2 },
  { kind: "circle", x: 2 * SLOT_W + (SLOT_W - SHAPE_PX) / 2 },
];

export function StarterPicker({ onPick }: Props) {
  return (
    <>
      {SLOTS.map(({ kind, x }) => (
        <div
          key={kind}
          style={{
            position: "absolute",
            left: x,
            top: SHAPE_Y,
            zIndex: 1,
          }}
        >
          <ShapeView
            kind={kind}
            size={SHAPE_PX}
            color={defaultColor(kind)}
            onClick={() => onPick(kind, { x, y: SHAPE_Y })}
          />
        </div>
      ))}
    </>
  );
}
