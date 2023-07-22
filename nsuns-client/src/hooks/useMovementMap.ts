import { Accessor, createMemo } from "solid-js";
import { Movement } from "../api";

export const useMovementMap = (movements: Accessor<Movement[]>) => {
  return createMemo(() => {
    const m: Record<number, Movement> = {};

    for (const movement of movements()) {
      m[movement.id] = movement;
    }

    return m;
  });
};
