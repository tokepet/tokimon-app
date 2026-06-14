import { useEffect, useState } from "react";
import { nextTarget, type Point } from "./randomTarget";

type Facing = "idle" | "left" | "right";

type WanderState = {
  target: Point;
  facing: Facing;
};

export function useWander(initial?: Point): WanderState {
  const [state, setState] = useState<WanderState>(() => ({
    target: initial ?? nextTarget(),
    facing: "idle",
  }));

  useEffect(() => {
    let timer = 0;
    const schedule = () => {
      const interval = 2000 + Math.random() * 2000;
      timer = window.setTimeout(() => {
        setState((current) => {
          const target = nextTarget();
          const facing =
            target.x > current.target.x
              ? "right"
              : target.x < current.target.x
                ? "left"
                : current.facing;

          return { target, facing };
        });
        schedule();
      }, interval);
    };

    schedule();
    return () => window.clearTimeout(timer);
  }, []);

  return state;
}
