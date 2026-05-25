import type { ShapeKind } from "../domain/ShapeKind";

type Props = {
  kind: ShapeKind;
  size: number;
  color: string;
  onClick?: () => void;
};

export function ShapeView({ kind, size, color, onClick }: Props) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      onClick={onClick}
      style={{
        cursor: onClick ? "pointer" : "default",
        display: "block",
      }}
    >
      {kind === "triangle" && <polygon points="50,5 95,90 5,90" fill={color} />}
      {kind === "square" && (
        <rect x="5" y="5" width="90" height="90" rx="6" fill={color} />
      )}
      {kind === "circle" && <circle cx="50" cy="50" r="45" fill={color} />}
    </svg>
  );
}
