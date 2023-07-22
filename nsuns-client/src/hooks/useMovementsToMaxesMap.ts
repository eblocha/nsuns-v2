import { Accessor, createMemo } from "solid-js";
import { Max } from "../api/maxes";
import { ProgramSet } from "../api";

export const useMovementsToMaxesMap = (maxes: Accessor<Max[]>) => {
  return createMemo(() => {
    const m: Record<number, Max[]> = {};
    for (const max of maxes()) {
      // maxes are in ascending timestamp order
      if (max.movementId in m) {
        m[max.movementId].push(max);
      } else {
        m[max.movementId] = [max];
      }
    }
    return m;
  });
};

export const getLatestMax = (movementsToMaxesMap: Record<number, Max[]>, set: ProgramSet): Max | undefined => {
  if (set.percentageOfMax) {
    const maxes = movementsToMaxesMap[set.percentageOfMax];
    return maxes[maxes.length - 1];
  } else {
    return undefined;
  }
}
