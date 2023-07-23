import { Accessor, createMemo } from "solid-js";
import { Reps } from "../api/reps";

export const useMovementsToRepsMap = (repsList: Accessor<Reps[]>) => {
  return createMemo(() => {
    const m: Record<number, Reps[]> = {};
    for (const reps of repsList()) {
      const current = m[reps.movementId];

      // maxes are in ascending timestamp order
      if (current) {
        current.push(reps);
      } else {
        m[reps.movementId] = [reps];
      }
    }
    return m;
  });
};
