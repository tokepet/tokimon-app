import { ShapeView } from "./ShapeView";
import type { ShapeKind } from "../domain/ShapeKind";
import { defaultColor } from "../domain/shapeDefaults";
import { SHAPE_PX, type Point } from "../motion/randomTarget";
import { useWander } from "../motion/useWander";

type Props = {
  kind: ShapeKind;
  startAt?: Point;
};

export function Wanderer({ kind, startAt }: Props) {
  const target = useWander(startAt);
  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        width: SHAPE_PX,
        height: SHAPE_PX,
        transform: `translate3d(${target.x}px, ${target.y}px, 0)`,
        transition: "transform 2s ease-in-out",
        willChange: "transform",
        backfaceVisibility: "hidden",
        contain: "layout paint",
        zIndex: 1,
      }}
    >
      <ShapeView kind={kind} size={SHAPE_PX} color={defaultColor(kind)} />
    </div>
  );
}
