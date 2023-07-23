import { Component, For, createMemo } from "solid-js";
import { useProgram } from "../context/ProgramProvider";
import { Max } from "../../../api/maxes";
import { Graph } from "../../../graph/Graph";
import { Movement } from "../../../api";
import { Reps } from "../../../api/reps";

type Amount = {
  amount: number;
};

const StatCard: Component<{ title: string; amount?: number }> = (props) => {
  return (
    <div class="rounded flex flex-col border border-gray-600 text-xl">
      <div class="text-center flex-shrink-0 p-3 border-b border-gray-600 bg-gray-900">
        {props.title}
      </div>
      <div class="flex-grow flex flex-col items-center justify-center p-6 text-4xl">
        {props.amount ?? <span class="italic text-gray-500">None</span>}
      </div>
    </div>
  );
};

const LastAmountCard: Component<{
  stats: Amount[];
  title: string;
}> = (props) => {
  const last = () => props.stats[props.stats.length - 1];

  return <StatCard title={props.title} amount={last()?.amount} />;
};

const StatRow: Component<{ stats: Amount[]; title: string }> = (props) => {
  return (
    <div class="grid grid-cols-4 gap-4">
      <LastAmountCard {...props} />
      <div class="col-span-3 h-32 text-blue-500 p-1 mt-auto">
        <Graph
          data={props.stats?.map((stat, index) => ({
            x: index,
            y: stat.amount,
          }))}
          weight={4}
          fillOpacity="10%"
        />
      </div>
    </div>
  );
};

type MaxRowData = {
  movement: Movement;
  maxes: Max[];
  reps: Reps[];
};

export const DataList: Component = () => {
  const {
    movementsToMaxesMap,
    movementsToRepsMap,
    movementMap,
    relevantMovements,
  } = useProgram();

  const maxesToShow = createMemo<MaxRowData[]>(() => {
    const mm = movementMap();
    const m2m = movementsToMaxesMap();
    const m2r = movementsToRepsMap();

    return relevantMovements().map((movementId) => ({
      movement: mm[movementId],
      maxes: m2m[movementId] ?? [],
      reps: m2r[movementId] ?? [],
    }));
  });

  return (
    <ul>
      <For each={maxesToShow()}>
        {({ movement, maxes, reps }) => (
          <li class="w-full grid grid-cols-2 gap-4 mt-2">
            <StatRow title={movement?.name} stats={maxes} />
            <StatRow title={movement?.name} stats={reps} />
          </li>
        )}
      </For>
    </ul>
  );
};
