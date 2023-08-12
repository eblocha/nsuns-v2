import { Component, Show, createEffect, on, onCleanup } from "solid-js";
import {
  currentSet,
  day,
  direction,
  nextDay,
  prevDay,
  setCurrentSet,
  setDirection,
} from "./state";
import { SetList } from "./SetList";
import style from "./AnimatedSetList.module.css";
import { useProgram } from "./context/ProgramProvider";

const dayOut = () => (direction() < 0 ? nextDay(day()) : prevDay(day()));

export const AnimatedSetList: Component = () => {
  const { getSets, movementMap, movementsToMaxesMap } = useProgram();

  let timeout: ReturnType<typeof setTimeout>;

  createEffect(
    on(direction, () => {
      timeout = setTimeout(() => setDirection(0), 100);
      onCleanup(() => clearTimeout(timeout));
    })
  );

  return (
    <>
      <Show when={direction() !== 0}>
        {/* The day moving out of position */}
        <div
          class="overflow-hidden h-full w-full absolute top-0 left-0 px-1"
          classList={{
            [style["slide-out-right"]!]: direction() < 0,
            [style["slide-out-left"]!]: direction() > 0,
          }}
        >
          <SetList
            sets={getSets(dayOut())}
            currentSet={currentSet()}
            setCurrentSet={setCurrentSet}
            day={dayOut()}
            movementMap={movementMap()}
            movementsToMaxesMap={movementsToMaxesMap()}
          />
        </div>
      </Show>

      {/* The day that will end up in the final position */}
      <div
        class="overflow-hidden h-full w-full absolute top-0 left-0 px-1"
        classList={{
          [style["slide-in-right"]!]: direction() < 0,
          [style["slide-in-left"]!]: direction() > 0,
        }}
      >
        <SetList
          sets={getSets(day())}
          currentSet={currentSet()}
          setCurrentSet={setCurrentSet}
          day={day()}
          movementMap={movementMap()}
          movementsToMaxesMap={movementsToMaxesMap()}
        />
      </div>
    </>
  );
};
