import { useState } from "react";
import { StarterPicker } from "../components/StarterPicker";
import { Wanderer } from "../components/Wanderer";
import type { ShapeKind } from "../domain/ShapeKind";
import type { Point } from "../motion/randomTarget";

type AppState =
  | { phase: "selection" }
  | { phase: "wandering"; kind: ShapeKind; startAt: Point };

export function App() {
  const [state, setState] = useState<AppState>({ phase: "selection" });

  return (
    <>
      <div
        data-tauri-drag-region
        style={{ position: "absolute", inset: 0, zIndex: 0 }}
      />
      {state.phase === "selection" ? (
        <StarterPicker
          onPick={(kind, startAt) =>
            setState({ phase: "wandering", kind, startAt })
          }
        />
      ) : (
        <Wanderer kind={state.kind} startAt={state.startAt} />
      )}
    </>
  );
}
