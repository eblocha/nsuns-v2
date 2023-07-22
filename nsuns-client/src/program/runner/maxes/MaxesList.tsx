import { Component, For, createMemo } from "solid-js";
import { useProgram } from "../context/ProgramProvider";
import { Max } from "../../../api/maxes";
import { Graph } from "../../../graph/Graph";
import { Movement } from "../../../api";

const MaxCard: Component<{ maxes: Max[]; movement: Movement }> = (props) => {
  const lastMax = () => props.maxes[props.maxes.length - 1];

  return (
    <div class="rounded flex flex-col border border-gray-600 text-xl">
      <div class="text-center flex-shrink-0 p-3 border-b border-gray-600 bg-gray-900">
        {props.movement.name}
      </div>
      <div class="flex-grow flex flex-col items-center justify-center p-6 text-4xl">
        {lastMax()?.amount ?? <span class="italic text-gray-500">None</span>}
      </div>
    </div>
  );
};

type MaxRowData = {
  movement: Movement;
  maxes: Max[]
}

export const MaxesList: Component = () => {
  const {
    movementsToMaxesMap,
    movementMap,
    relevantMovements: movementsWithMaxInProgram,
  } = useProgram();

  const maxesToShow = createMemo<MaxRowData[]>(() => {
    const mm = movementMap();
    const m2m = movementsToMaxesMap();

    return movementsWithMaxInProgram().map((movementId) => ({
      movement: mm[movementId],
      maxes: m2m[movementId] ?? [],
    }));
  });

  return (
    <ul>
      <For each={maxesToShow()}>
        {({ movement, maxes }) => (
          <li class="w-full grid grid-cols-6 gap-4 mt-2">
            <MaxCard maxes={maxes} movement={movement} />
            <div class="col-span-5 h-32 text-blue-500 p-1 mt-auto">
              <Graph
                data={maxes?.map((max, index) => ({
                  x: index,
                  y: max.amount,
                }))}
                weight={4}
                fillOpacity="10%"
              />
            </div>
          </li>
        )}
      </For>
    </ul>
  );
};
