import { Accessor, createMemo } from "solid-js";
import { Max } from "../api/maxes";

export const useMovementsToMaxesMap = (maxes: Accessor<Max[]>) => {
  return createMemo(() => {
    const m: Record<number, Max> = {};
    for (const max of maxes()) {
      // maxes are in descending timestamp order
      if (max.movementId in m) continue;

      m[max.movementId] = max;
    }
    return m;
  });
};
