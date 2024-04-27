import { Component, For, Match, Show, Switch, createMemo } from "solid-js";
import { useProgram } from "../context/ProgramProvider";
import { Max } from "../../../api/maxes";
import { Graph } from "../../../graph/Graph";
import { Reps } from "../../../api/reps";
import { Input } from "../../../forms/Input";
import { useEditStat, CommonProps, EditableStatProps } from "../../../hooks/useEditStat";
import { SPINNER_DELAY_MS, createMinimumAsyncDelay, createSmartAsyncDelay } from "../../../hooks/asymmetricDelay";
import { Spinner } from "../../../icons/Spinner";

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

  const isUpdating = createMinimumAsyncDelay(() => mutation.isLoading, SPINNER_DELAY_MS);

  return (
    <div class="rounded flex flex-col border border-gray-600 text-xl">
      <div class="text-center flex-shrink-0 p-3 border-b border-gray-600 bg-gray-900">{props.movement?.name}</div>
      <div class="flex-grow flex flex-col items-center justify-center p-3 text-4xl">
        <Show
          when={!isUpdating()}
          fallback={
            <div class="h-14 flex flex-col justify-center">
              <Spinner class="animate-spin" />
            </div>
          }
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
              class="w-full h-14 ghost-input text-center"
              placeholder="Edit"
              disabled={isUpdating()}
              required={props.type === "max"}
              onBlur={reset}
            />
          </form>
        </Show>
      </div>
    </div>
  );
};

const LastAmountCard: Component<StatsProps> = (props) => {
  const last = () => props.stats[props.stats.length - 1];

  return (
    // @ts-expect-error: complaining about tagged union types
    <EditableCard
      stat={last()}
      type={props.type}
      movement={props.movement}
      profileId={props.profileId}
    />
  );
};

const StatRow: Component<StatsProps> = (props) => {
  const points = createMemo(
    () =>
      props.stats?.map((stat, index) => ({
        x: index,
        y: stat.amount ?? 0,
      }))
  );

  return (
    <div class="grid grid-cols-4 gap-4">
      <LastAmountCard {...props} />
      <div class="col-span-3 h-32 text-blue-500 p-1 mt-auto">
        <Graph
          data={points()}
          weight={4}
          fillOpacity="10%"
          softYMinimum={props.type === "reps" ? 0 : undefined}
        />
      </div>
    </div>
  );
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

  const delayedIsLoading = createSmartAsyncDelay(isLoading);

  return (
    <Switch>
      <Match when={delayedIsLoading()}>
        <Loading />
      </Match>
      <Match when={isSuccess()}>
        <div class="flex-grow overflow-auto grid grid-cols-1 2xl:grid-cols-2 mx-14 lg:mx-0 mt-4 lg:mt-0">
          <div>
            <div class="text-3xl">Maxes</div>
            <ul>
              <For each={relevantMovements()}>
                {(movementId) => (
                  <li class="w-full mt-2">
                    <StatRow
                      movement={movementMap()[movementId]}
                      stats={movementsToMaxesMap()[movementId] ?? []}
                      type="max"
                      profileId={profileId()}
                    />
                  </li>
                )}
              </For>
            </ul>
          </div>
          <div>
            <div class="text-3xl mt-2 2xl:mt-0">Reps</div>
            <ul>
              <For each={relevantMovements()}>
                {(movementId) => (
                  <li class="w-full mt-2">
                    <StatRow
                      movement={movementMap()[movementId]}
                      stats={movementsToRepsMap()[movementId] ?? []}
                      type="reps"
                      profileId={profileId()}
                    />
                  </li>
                )}
              </For>
            </ul>
          </div>
        </div>
      </Match>
    </Switch>
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
