import { PetView } from "./PetView";
import { findPet } from "../domain/petCatalog";
import {
  resolveTokimonBehaviorState,
  resolveTokimonVisualState,
  type TokimonPersistentState,
  type TokimonReaction,
} from "../domain/tokimonState";
import { SHAPE_PX, type Point } from "../motion/randomTarget";
import { useWander, WANDER_MOVE_MS } from "../motion/useWander";

type Props = {
  petId: string;
  startAt?: Point;
  petState?: TokimonPersistentState | null;
  reaction?: TokimonReaction | null;
  onInteract?: () => void;
};

export function Wanderer({
  petId,
  startAt,
  petState,
  reaction,
  onInteract,
}: Props) {
  const { target, facing } = useWander(startAt);
  const pet = findPet(petId);
  if (!pet) return null;

  const behavior = resolveTokimonBehaviorState({
    pet: petState,
    facing,
    reaction: reaction ?? null,
  });
  const visualState = resolveTokimonVisualState({ pet, behavior, facing });
  const flipX = visualState === "walk" && facing === "left";

  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        width: SHAPE_PX,
        height: SHAPE_PX,
        transform: `translate3d(${target.x}px, ${target.y}px, 0)`,
        transition: `transform ${WANDER_MOVE_MS}ms ease-in-out`,
        willChange: "transform",
        backfaceVisibility: "hidden",
        contain: "layout paint",
        zIndex: 1,
      }}
    >
      <PetView
        pet={pet}
        state={visualState}
        size={SHAPE_PX}
        onClick={onInteract}
        flipX={flipX}
      />
    </div>
  );
}
