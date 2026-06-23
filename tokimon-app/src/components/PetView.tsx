import { SpriteSheet } from "./SpriteSheet";
import type { PetDef, PetState } from "../domain/petCatalog";

type Props = {
  pet: PetDef;
  state?: PetState;
  size?: number;
  animate?: boolean;
  onClick?: () => void;
  flipX?: boolean;
};

export function PetView({
  pet,
  state = "idle",
  size,
  animate = true,
  onClick,
  flipX = false,
}: Props) {
  const sprite = pet.states[state] ?? pet.states.idle;
  if (!sprite) return null;
  const displaySize = size ?? pet.displaySize;

  return (
    <div
      onClick={onClick}
      style={{
        width: displaySize,
        height: displaySize,
        cursor: onClick ? "pointer" : "default",
      }}
    >
      <SpriteSheet
        src={sprite.src}
        frameCount={sprite.frameCount}
        frameWidth={sprite.frameWidth}
        frameHeight={sprite.frameHeight}
        fps={sprite.fps}
        displaySize={displaySize}
        animate={animate}
        flipX={flipX}
      />
    </div>
  );
}
