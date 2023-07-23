import { Accessor, createMemo } from "solid-js";
import { Max } from "../api/maxes";
import { ProgramSet } from "../api";

export const useMovementsToMaxesMap = (maxes: Accessor<Max[]>) => {
  return createMemo(() => {
    const m: Record<string, Max[]> = {};
    for (const max of maxes()) {
      const current = m[max.movementId];
      // maxes are in ascending timestamp order
      if (current) {
        current.push(max);
      } else {
        m[max.movementId] = [max];
      }
    }
    return m;
  });
};

export const getLatestMax = (
  movementsToMaxesMap: Record<string, Max[]>,
  set: ProgramSet
): Max | undefined => {
  if (set.percentageOfMax) {
    const maxes = movementsToMaxesMap[set.percentageOfMax];
    return maxes?.[maxes.length - 1];
  } else {
    return undefined;
  }
};
