import { Component, Match, Switch, createSignal } from "solid-js";
import { Max } from "../api/maxes";
import { Reps } from "../api/reps";
import { Movement } from "../api";
import { plural } from "../util/setDisplay";
import { Plus } from "../icons/Plus";
import { Graph } from "../graph/Graph";
import { UpdateMaxes } from "./UpdateMaxes";
import { UndoUpdate } from "./UndoUpdate";
import { NewMaxForm } from "./log/NewMaxForm";
import { NewRepsForm } from "./log/NewRepsForm";

const displayAmount = (amount?: number) => {
  return amount !== undefined ? `${amount} lb${plural(amount)}` : "None";
};

const displayReps = (amount?: number) => {
  return amount !== undefined ? `${amount} rep${plural(amount)}` : "no reps";
};

export const MovementData: Component<{
  maxes: Max[];
  reps: Reps[];
  movement: Movement;
  profileId: string;
}> = (props) => {
  const latestMax = () =>
    displayAmount(props.maxes[props.maxes.length - 1]?.amount);
  const latestReps = () =>
    displayReps(props.reps[props.reps.length - 1]?.amount ?? undefined);

  const [showForm, setShowForm] = createSignal<"max" | "reps" | null>(null);

  return (
    <div class="flex flex-col gap-2">
      <div class="grid grid-cols-5">
        <div class="col-span-2">
          <p>Max: {latestMax()}</p>
          <p>{latestReps()}</p>
        </div>
        <div class="h-20 text-blue-500 col-span-3 p-1 border-l border-b border-gray-600">
          <Graph
            data={props.maxes.map((max, index) => ({
              x: index,
              y: max.amount,
            }))}
            weight={props.maxes.length > 1 ? 3 : 5}
            fillOpacity="10%"
          />
        </div>
      </div>
      <Switch>
        <Match when={showForm() === null}>
          <div class="flex flex-row items-stretch gap-2">
            <button
              class="text-button-outline flex flex-row items-center justify-center gap-2"
              onClick={() => setShowForm("max")}
            >
              <Plus />
              Max
            </button>
            <button
              class="text-button-outline flex flex-row items-center justify-center gap-2"
              onClick={() => setShowForm("reps")}
            >
              <Plus />
              Reps
            </button>
            <UpdateMaxes
              movementId={props.movement.id}
              profileId={props.profileId}
            />
            <UndoUpdate
              movementId={props.movement.id}
              profileId={props.profileId}
            />
          </div>
        </Match>
        <Match when={showForm() === "max"}>
          <NewMaxForm
            profileId={props.profileId}
            movementId={props.movement.id}
            onClose={() => setShowForm(null)}
          />
        </Match>
        <Match when={showForm() === "reps"}>
          <NewRepsForm
            profileId={props.profileId}
            movementId={props.movement.id}
            onClose={() => setShowForm(null)}
          />
        </Match>
      </Switch>
    </div>
  );
};
