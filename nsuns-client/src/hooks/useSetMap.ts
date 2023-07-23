import { Accessor, createMemo } from "solid-js";
import { ProgramSet } from "../api";
import { DayName, dayNames } from "../util/days";

export const useSetMap = (sets: Accessor<ProgramSet[]>) => {
  return createMemo(() => {
    const m = {} as Record<DayName, ProgramSet[]>;
    dayNames.forEach((name, index) => {
      m[name] = sets().filter((set) => set.day === index);
    });
    return m;
  });
};
