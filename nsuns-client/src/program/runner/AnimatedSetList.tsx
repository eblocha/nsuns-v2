import { Component, Show, createEffect, on, onCleanup } from "solid-js";
import { Movement, ProgramSet } from "../../api";
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
import { Max } from "../../api/maxes";

const dayNameOut = () => (direction() < 0 ? nextDayName() : prevDayName());

export const AnimatedSetList: Component<{
  setMap: Record<string, ProgramSet[]>;
  movementMap?: Record<number, Movement>;
  movementsToMaxesMap?: Record<number, Max[]>;
}> = (props) => {
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
            sets={props.setMap[dayNameOut()]}
            currentSet={currentSet()}
            setCurrentSet={setCurrentSet}
            day={dayNameOut()}
            movementMap={props.movementMap}
            movementsToMaxesMap={props.movementsToMaxesMap}
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
          sets={props.setMap[dayName()]}
          currentSet={currentSet()}
          setCurrentSet={setCurrentSet}
          day={dayName()}
          movementMap={props.movementMap}
          movementsToMaxesMap={props.movementsToMaxesMap}
        />
      </div>
    </>
  );
};
