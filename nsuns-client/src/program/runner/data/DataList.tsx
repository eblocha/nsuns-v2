import {
  Component,
  For,
  JSX,
  createEffect,
  createMemo,
  createRenderEffect,
} from "solid-js";
import { useProgram } from "../context/ProgramProvider";
import { Max } from "../../../api/maxes";
import { Graph } from "../../../graph/Graph";
import { Movement } from "../../../api";
import { Reps } from "../../../api/reps";
import { createControl } from "../../../hooks/forms";
import { Input } from "../../../forms/Input";
import { useUpdateMaxMutation } from "../../../hooks/queries/maxes";
import { useUpdateRepsMutation } from "../../../hooks/queries/reps";

type Stats =
  | {
      stats: Max[];
      title: string;
      type: "max";
    }
  | {
      stats: Reps[];
      title: string;
      type: "reps";
    };

type EditableStatProps =
  | {
      title: string;
      stat: Max;
      type: "max";
    }
  | {
      title: string;
      stat: Reps;
      type: "reps";
    };

const StatCard: Component<{
  title: string;
  children?: JSX.Element;
  loading?: boolean;
}> = (props) => {
  return (
    <div class="rounded flex flex-col border border-gray-600 text-xl">
      <div class="text-center flex-shrink-0 p-3 border-b border-gray-600 bg-gray-900">
        {props.title}
      </div>
      <div
        class="flex-grow flex flex-col items-center justify-center p-3 text-4xl"
        classList={{
          shimmer: props.loading,
        }}
      >
        {props.children}
      </div>
    </div>
  );
};

const EditableCard: Component<EditableStatProps> = (props) => {
  const amount = createControl(props.stat?.amount.toString() || "");

  const reset = () => {
    amount.reset(props.stat?.amount.toString());
  };

  createRenderEffect(reset);

  const maxesMutation = useUpdateMaxMutation(() => props.stat?.profileId, {
    onError: () => reset(),
  });
  const repsMutation = useUpdateRepsMutation(() => props.stat?.profileId, {
    onError: () => reset(),
  });

  const mutation = () => (props.type === "max" ? maxesMutation : repsMutation);

  const onSubmit = () => {
    const amt = amount.value();
    if (mutation().isLoading || !amt) return;

    const parsed = parseInt(amt);

    if (parsed === props.stat?.amount) return;

    mutation().mutate({
      ...props.stat,
      amount: parsed,
    });
  };

  return (
    <StatCard title={props.title} loading={mutation().isLoading}>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit();
        }}
      >
        <Input
          control={amount}
          required={true}
          type="number"
          min={0}
          class="w-full h-full ghost-input text-center"
          placeholder="Edit"
          disabled={mutation().isLoading}
          onBlur={reset}
        />
      </form>
    </StatCard>
  );
};

const LastAmountCard: Component<Stats> = (props) => {
  const last = () => props.stats[props.stats.length - 1];

  return <EditableCard title={props.title} stat={last()} type={props.type} />;
};

const StatRow: Component<Stats> = (props) => {
  const points = createMemo(() =>
    props.stats?.map((stat, index) => ({
      x: index,
      y: stat.amount,
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
  movement: Movement;
  maxes: Reps[];
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
            <StatRow title={movement?.name} stats={maxes} type="max" />
            <StatRow title={movement?.name} stats={reps} type="reps" />
          </li>
        )}
      </For>
    </ul>
  );
};
