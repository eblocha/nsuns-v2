import { Component, For, JSX, Setter, Show, createMemo } from "solid-js";
import { Movement, ProgramSet } from "../../api";
import {
  getSections,
  plural,
  resolvedWeightDisplay,
} from "../../util/setDisplay";
import { Max } from "../../api/maxes";
import { day, goToToday, today } from "./state";
import { getLatestMax } from "../../hooks/useMovementsToMaxesMap";
import { Moon } from "../../icons/Moon";

export const displaySet = (set: ProgramSet, max?: number): JSX.Element => {
  const repsComponent =
    set.reps != null
      ? ` for ${set.reps}${set.repsIsMinimum ? "+" : ""} rep${
          set.repsIsMinimum ? "s" : plural(set.reps)
        }`
      : "";

  const weightComponent = resolvedWeightDisplay(set, max);

  return set.amount
    ? `${weightComponent}${repsComponent}`
    : set.description || (
        <span class="italic text-gray-200">No description</span>
      );
};

export const SetList: Component<{
  sets?: ProgramSet[];
  currentSet: number;
  setCurrentSet: Setter<number>;
  movementMap?: Record<number, Movement>;
  movementsToMaxesMap?: Record<number, Max[]>;
  day: string;
}> = (props) => {
  const sections = createMemo(() =>
    getSections(props.sets ?? [], props.movementMap ?? {})
  );

  return (
    <div class="w-full h-full flex flex-col border rounded border-gray-700 overflow-hidden">
      <div class="flex flex-row items-center border-b border-gray-700 p-2 bg-gray-900">
        <h2 class="text-xl">{props.day}</h2>
        <button
          class="text-button ml-auto text-sm"
          onClick={goToToday}
          disabled={day() === today()}
        >
          Go To Today
        </button>
      </div>
      <div class="overflow-hidden flex-grow">
        <div class="h-full w-full overflow-auto">
          <ul class="p-2">
            <For
              each={sections()}
              fallback={
                <div class="w-full flex flex-col items-center justify-center text-lg">
                  <span class="italic">Rest day</span>
                  <Moon class="mt-4 text-2xl" />
                </div>
              }
            >
              {(section, index) => (
                <>
                  <Show when={index() !== 0}>
                    <hr class="border-gray-600" />
                  </Show>
                  <h3
                    class="text-lg mb-1"
                    title={section.movement.description || undefined}
                  >
                    {section.movement.name}
                  </h3>
                  <For each={section.sets}>
                    {({ set, index }) => (
                      <li class="w-full mb-1 rounded">
                        <button
                          onClick={() => props.setCurrentSet(index)}
                          class="text-button rounded w-full text-left"
                          classList={{
                            "text-button": props.currentSet !== index,
                            "primary-button": props.currentSet === index,
                          }}
                        >
                          {displaySet(
                            set,
                            props.movementsToMaxesMap &&
                              getLatestMax(props.movementsToMaxesMap, set)
                                ?.amount
                          )}
                        </button>
                      </li>
                    )}
                  </For>
                </>
              )}
            </For>
          </ul>
        </div>
      </div>
    </div>
  );
};
