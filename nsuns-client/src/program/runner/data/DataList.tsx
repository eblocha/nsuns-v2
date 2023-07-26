import { Component, For, Match, Switch, createMemo } from "solid-js";
import { useProgram } from "../context/ProgramProvider";
import { Max } from "../../../api/maxes";
import { Graph } from "../../../graph/Graph";
import { Movement } from "../../../api";
import { Reps } from "../../../api/reps";
import { Input } from "../../../forms/Input";
import { useEditStat, CommonProps, EditableStatProps } from "../../../hooks/useEditStat";

type StatsProps = CommonProps &
  (
    | {
        stats: Max[];
        type: "max";
      }
    | {
        stats: Reps[];
        type: "reps";
      }
  );

const EditableCard: Component<EditableStatProps> = (props) => {
  const { amount, onSubmit, reset, mutation } = useEditStat(props);

  return (
    <div class="rounded flex flex-col border border-gray-600 text-xl">
      <div class="text-center flex-shrink-0 p-3 border-b border-gray-600 bg-gray-900">
        {props.movement?.name}
      </div>
      <div
        class="flex-grow flex flex-col items-center justify-center p-3 text-4xl"
        classList={{
          shimmer: mutation.isLoading,
        }}
      >
        <form
          onSubmit={(e) => {
            e.preventDefault();
            onSubmit();
          }}
        >
          <Input
            control={amount}
            type="number"
            min={0}
            class="w-full h-full ghost-input text-center"
            placeholder="Edit"
            disabled={mutation.isLoading}
            required={props.type === "max"}
            onBlur={reset}
          />
        </form>
      </div>
    </div>
  );
};

const LastAmountCard: Component<StatsProps> = (props) => {
  const last = () => props.stats[props.stats.length - 1];

  return (
    // @ts-ignore: complaining about tagged union types
    <EditableCard
      stat={last()}
      type={props.type}
      movement={props.movement}
      profileId={props.profileId}
    />
  );
};

const StatRow: Component<StatsProps> = (props) => {
  const points = createMemo(() =>
    props.stats?.map((stat, index) => ({
      x: index,
      y: stat.amount ?? 0,
    }))
  );

  return (
    <div class="grid grid-cols-4 gap-4">
      <LastAmountCard {...props} />
      <div class="col-span-3 h-32 text-blue-500 p-1 mt-auto">
        <Graph data={points()} weight={4} fillOpacity="10%" />
      </div>
    </div>
  );
};

type MaxRowData = {
  movement?: Movement;
  maxes: Reps[];
  reps: Reps[];
};

export const DataList: Component = () => {
  const {
    movementsToMaxesMap,
    movementsToRepsMap,
    movementMap,
    relevantMovements,
    profileId,
    queryState: { isLoading, isSuccess },
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
      <Switch>
        <Match when={isLoading()}>
          <Loading />
        </Match>
        <Match when={isSuccess()}>
          <For each={maxesToShow()}>
            {({ movement, maxes, reps }) => (
              <li class="w-full grid grid-cols-2 gap-4 mt-2">
                {/* @ts-ignore: complaining about tagged union types */}
                <StatRow
                  movement={movement}
                  stats={maxes}
                  type="max"
                  profileId={profileId()}
                />
                <StatRow
                  movement={movement}
                  stats={reps}
                  type="reps"
                  profileId={profileId()}
                />
              </li>
            )}
          </For>
        </Match>
      </Switch>
    </ul>
  );
};

const Loading: Component = () => {
  return (
    <For each={[1, 2, 3, 4]}>
      {() => (
        <li class="w-full grid grid-cols-2 gap-4 mt-4">
          <div class="rounded shimmer h-32" />
          <div class="rounded shimmer h-32" />
        </li>
      )}
    </For>
  );
};
