import { useEffect, useRef, useState } from "react";
import { nextTarget, type Point } from "./randomTarget";
import type { TokimonFacing } from "../domain/tokimonState";

export const WANDER_MOVE_MS = 3_500;

const WANDER_PAUSE_MIN_MS = 1_500;
const WANDER_PAUSE_RANDOM_MS = 2_500;

type WanderState = {
  target: Point;
  facing: TokimonFacing;
};

export function useWander(initial?: Point): WanderState {
  const [state, setState] = useState<WanderState>(() => ({
    target: initial ?? nextTarget(),
    facing: "idle",
  }));
  const stateRef = useRef(state);

  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  useEffect(() => {
    let scheduleTimer = 0;
    let stopTimer = 0;

    const schedule = () => {
      const interval = WANDER_MOVE_MS + WANDER_PAUSE_MIN_MS + Math.random() * WANDER_PAUSE_RANDOM_MS;
      scheduleTimer = window.setTimeout(() => {
        const current = stateRef.current;
        const target = nextTarget();
        const facing = nextFacing(current, target);
        const next = { target, facing };

        stateRef.current = next;
        setState(next);

        window.clearTimeout(stopTimer);
        if (facing !== "idle") {
          stopTimer = window.setTimeout(() => {
            setState((latest) => {
              if (latest.target.x !== target.x || latest.target.y !== target.y) {
                return latest;
              }
              const stopped = { ...latest, facing: "idle" as const };
              stateRef.current = stopped;
              return stopped;
            });
          }, WANDER_MOVE_MS);
        }

        schedule();
      }, interval);
    };

    schedule();
    return () => {
      window.clearTimeout(scheduleTimer);
      window.clearTimeout(stopTimer);
    };
  }, []);

  return state;
}

function nextFacing(current: WanderState, target: Point): TokimonFacing {
  const dx = target.x - current.target.x;
  const dy = target.y - current.target.y;

  if (Math.abs(dx) > 1) return dx > 0 ? "right" : "left";
  if (Math.abs(dy) > 1) return current.facing === "idle" ? "right" : current.facing;
  return "idle";
}
