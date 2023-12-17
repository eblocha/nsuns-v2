import { Component, For, JSX, Setter, Show, createMemo } from "solid-js";
import { Movement, ProgramSet } from "../../api";
import { getSections, plural, resolvedWeightDisplay } from "../../util/setDisplay";
import { Max } from "../../api/maxes";
import { day, goToToday, today } from "./state";
import { getLatestMax } from "../../hooks/useMovementsToMaxesMap";
import { Moon } from "../../icons/Moon";
import { Day, dayNames } from "../../util/days";
import { SetComponent } from "./Set";

export const displaySet = (set: ProgramSet, max?: number): JSX.Element => {
  const repsComponent =
    set.reps != null
      ? ` for ${set.reps}${set.repsIsMinimum ? "+" : ""} rep${set.repsIsMinimum ? "s" : plural(set.reps)}`
      : "";

  const weightComponent = resolvedWeightDisplay(set, max);

  return set.amount
    ? `${weightComponent}${repsComponent}`
    : set.description || <span class="italic text-gray-200">No description</span>;
};

export const SetList: Component<{
  sets?: ProgramSet[];
  currentSet: number;
  setCurrentSet: Setter<number>;
  movementMap?: Record<string, Movement>;
  movementsToMaxesMap?: Record<string, Max[]>;
  day: Day;
}> = (props) => {
  const sections = createMemo(() => getSections(props.sets ?? [], props.movementMap ?? {}));

  return (
    <div class="w-full h-full flex flex-col border rounded border-gray-700 overflow-hidden">
      <div class="flex flex-row items-center border-b border-gray-700 p-2 bg-gray-900">
        <h2 class="text-3xl">{dayNames[props.day]}</h2>
        <button
          class="text-button ml-auto"
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
                <div class="w-full flex flex-col items-center justify-center text-xl">
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
                    class="text-2xl mb-1"
                    title={section.movement.description || undefined}
                  >
                    {section.movement.name}
                  </h3>
                  <For each={section.sets}>
                    {({ set, index }) => (
                      <li class="w-full mb-1 rounded text-xl">
                        <SetComponent
                          onClick={() => props.setCurrentSet(index)}
                          isActive={props.currentSet === index}
                        >
                          {displaySet(
                            set,
                            props.movementsToMaxesMap && getLatestMax(props.movementsToMaxesMap, set)?.amount
                          )}
                        </SetComponent>
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
