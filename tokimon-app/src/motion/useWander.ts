import { useEffect, useState } from "react";
import { nextTarget, type Point } from "./randomTarget";

export function useWander(initial?: Point): Point {
  const [target, setTarget] = useState<Point>(() => initial ?? nextTarget());

  useEffect(() => {
    let timer = 0;
    const schedule = () => {
      const interval = 2000 + Math.random() * 2000;
      timer = window.setTimeout(() => {
        setTarget(nextTarget());
        schedule();
      }, interval);
    };
    schedule();
    return () => window.clearTimeout(timer);
  }, []);

  return target;
}
