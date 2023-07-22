import { Component, Show, createEffect, on, onCleanup } from "solid-js";
import {
  currentSet,
  dayName,
  direction,
  nextDayName,
  prevDayName,
  setCurrentSet,
  setDirection,
} from "./state";
import { SetList } from "./SetList";
import style from "./AnimatedSetList.module.css";
import { useProgram } from "./context/ProgramProvider";

const dayNameOut = () => (direction() < 0 ? nextDayName() : prevDayName());

export const AnimatedSetList: Component = (props) => {
  const { setMap, movementMap, movementsToMaxesMap } = useProgram();

  let timeout: number;

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
            [style["slide-out-right"]]: direction() < 0,
            [style["slide-out-left"]]: direction() > 0,
          }}
        >
          <SetList
            sets={setMap()[dayNameOut()]}
            currentSet={currentSet()}
            setCurrentSet={setCurrentSet}
            day={dayNameOut()}
            movementMap={movementMap()}
            movementsToMaxesMap={movementsToMaxesMap()}
          />
        </div>
      </Show>

      {/* The day that will end up in the final position */}
      <div
        class="overflow-hidden h-full w-full absolute top-0 left-0 px-1"
        classList={{
          [style["slide-in-right"]]: direction() < 0,
          [style["slide-in-left"]]: direction() > 0,
        }}
      >
        <SetList
          sets={setMap()[dayName()]}
          currentSet={currentSet()}
          setCurrentSet={setCurrentSet}
          day={dayName()}
          movementMap={movementMap()}
          movementsToMaxesMap={movementsToMaxesMap()}
        />
      </div>
    </>
  );
};
