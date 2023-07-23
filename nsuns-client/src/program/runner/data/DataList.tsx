import {
  Component,
  For,
  Match,
  Switch,
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
import {
  useCreateMaxMutation,
  useUpdateMaxMutation,
} from "../../../hooks/queries/maxes";
import {
  useCreateRepsMutation,
  useUpdateRepsMutation,
} from "../../../hooks/queries/reps";
import { createMutation } from "@tanstack/solid-query";

type CommonProps = {
  movement?: Movement;
  profileId: string;
};

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

type EditableStatProps = CommonProps &
  (
    | {
        stat?: Max;
        type: "max";
      }
    | {
        stat?: Reps;
        type: "reps";
      }
  );

const EditableCard: Component<EditableStatProps> = (props) => {
  const amount = createControl(props.stat?.amount.toString() || "");

  const reset = () => {
    amount.reset(props.stat?.amount.toString());
  };

  createRenderEffect(reset);

  const profileId = () => props.profileId;
  const options = {
    onError: reset,
  };

  const updateMax = useUpdateMaxMutation(profileId, options);
  const createMax = useCreateMaxMutation(profileId, options);

  const updateReps = useUpdateRepsMutation(profileId, options);
  const createReps = useCreateRepsMutation(profileId, options);

  const mutation = createMutation({
    mutationFn: async ({
      amount,
      movement,
    }: {
      amount: number;
      movement: Movement;
    }) => {
      if (props.stat && props.type === "max") {
        updateMax.mutate({
          id: props.stat.id,
          amount,
        });
      } else if (props.stat && props.type === "reps") {
        updateReps.mutate({
          id: props.stat.id,
          amount,
        });
      } else if (props.type === "max") {
        createMax.mutate({
          amount,
          movementId: movement.id,
          profileId: props.profileId,
        });
      } else if (props.type === "reps") {
        createReps.mutate({
          amount,
          movementId: movement.id,
          profileId: props.profileId,
        });
      }
    },
  });

  const onSubmit = () => {
    const amt = amount.value();
    if (mutation.isLoading || !amt || !props.movement) return;

    const parsed = parseInt(amt);

    if (parsed === props.stat?.amount) return;

    mutation.mutate({ amount: parsed, movement: props.movement });
  };

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
