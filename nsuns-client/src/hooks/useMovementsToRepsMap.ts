import { Accessor, createMemo } from "solid-js";
import { Reps } from "../api/reps";
import { ProgramSet } from "../api";

export const useMovementsToRepsMap = (repsList: Accessor<Reps[]>) => {
  return createMemo(() => {
    const m: Record<string, Reps[]> = {};
    for (const reps of repsList()) {
      const current = m[reps.movementId];

      // reps are in ascending timestamp order
      if (current) {
        current.push(reps);
      } else {
        m[reps.movementId] = [reps];
      }
    }
    return m;
  });
};

export const getLatestReps = (movementsToRepsMap: Record<string, Reps[]>, set: ProgramSet): Reps | undefined => {
  if (set.percentageOfMax) {
    const reps = movementsToRepsMap[set.percentageOfMax];
    return reps?.[reps.length - 1];
  } else {
    return undefined;
  }
};

