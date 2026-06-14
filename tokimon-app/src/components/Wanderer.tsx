import { PetView } from "./PetView";
import { findPet } from "../domain/petCatalog";
import { SHAPE_PX, type Point } from "../motion/randomTarget";
import { useWander } from "../motion/useWander";

type Props = {
  petId: string;
  startAt?: Point;
};

export function Wanderer({ petId, startAt }: Props) {
  const { target, facing } = useWander(startAt);
  const pet = findPet(petId);
  if (!pet) return null;

  // Sprite sheet labels are opposite to their visual facing direction.
  const petState =
    facing === "left" ? "walkRight" : facing === "right" ? "walkLeft" : "idle";

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
      <PetView pet={pet} state={petState} size={SHAPE_PX} />
    </div>
  );
}
