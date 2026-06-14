import { useEffect, useState } from "react";

type Props = {
  src: string;
  frameCount: number;
  frameWidth: number;
  frameHeight: number;
  fps: number;
  displaySize: number;
  animate?: boolean;
  pixelated?: boolean;
};

export function SpriteSheet({
  src,
  frameCount,
  frameWidth,
  frameHeight,
  fps,
  displaySize,
  animate = true,
  pixelated = true,
}: Props) {
  const [frame, setFrame] = useState(0);

  useEffect(() => {
    if (!animate || frameCount <= 1) {
      setFrame(0);
      return;
    }

    const id = window.setInterval(
      () => setFrame((f) => (f + 1) % frameCount),
      1000 / fps,
    );
    return () => window.clearInterval(id);
  }, [animate, frameCount, fps]);

  const aspect = frameWidth / frameHeight;
  const frameDisplayH = aspect >= 1 ? displaySize / aspect : displaySize;
  const frameDisplayW = aspect >= 1 ? displaySize : displaySize * aspect;
  const offsetX = (displaySize - frameDisplayW) / 2;
  const offsetY = (displaySize - frameDisplayH) / 2;
  const scale = frameDisplayH / frameHeight;
  const sheetDisplayW = frameWidth * frameCount * scale;

  return (
    <div
      style={{
        width: displaySize,
        height: displaySize,
        overflow: "hidden",
        position: "relative",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: offsetX,
          top: offsetY,
          width: frameDisplayW,
          height: frameDisplayH,
          overflow: "hidden",
        }}
      >
        <img
          src={src}
          width={sheetDisplayW}
          height={frameDisplayH}
          alt=""
          draggable={false}
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            transform: `translateX(${-frame * frameWidth * scale}px)`,
            imageRendering: pixelated ? "pixelated" : "auto",
            display: "block",
            userSelect: "none",
            pointerEvents: "none",
          }}
        />
      </div>
    </div>
  );
}
