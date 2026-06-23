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
  flipX?: boolean;
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
  flipX = false,
}: Props) {
  const [frame, setFrame] = useState(0);
  const safeFrameCount = Math.max(1, Math.floor(frameCount));
  const safeFps = Math.max(0.1, fps);

  useEffect(() => {
    setFrame(0);
  }, [src, frameWidth, frameHeight, safeFrameCount]);

  useEffect(() => {
    if (!animate || safeFrameCount <= 1) {
      setFrame(0);
      return;
    }

    const id = window.setInterval(
      () => setFrame((f) => (f + 1) % safeFrameCount),
      1000 / safeFps,
    );
    return () => window.clearInterval(id);
  }, [animate, safeFrameCount, safeFps]);

  const aspect = frameWidth / frameHeight;
  const frameDisplayH = aspect >= 1 ? displaySize / aspect : displaySize;
  const frameDisplayW = aspect >= 1 ? displaySize : displaySize * aspect;
  const offsetX = (displaySize - frameDisplayW) / 2;
  const offsetY = (displaySize - frameDisplayH) / 2;
  const scale = frameDisplayH / frameHeight;
  const sheetDisplayW = frameWidth * safeFrameCount * scale;
  const visibleFrame = frame % safeFrameCount;

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
          transform: flipX ? "scaleX(-1)" : "none",
          transformOrigin: "center",
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
            transform: `translateX(${-visibleFrame * frameWidth * scale}px)`,
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
