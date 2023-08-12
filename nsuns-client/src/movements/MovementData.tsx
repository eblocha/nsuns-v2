import { Component, Match, Show, Switch, createEffect, createSignal, onCleanup } from "solid-js";
import { Max } from "../api/maxes";
import { Reps } from "../api/reps";
import { Movement } from "../api";
import { Plus } from "../icons/Plus";
import { Graph } from "../graph/Graph";
import { UpdateMaxes } from "./UpdateMaxes";
import { UndoUpdate } from "./UndoUpdate";
import { NewMaxForm } from "./log/NewMaxForm";
import { NewRepsForm } from "./log/NewRepsForm";
import { EditableStatProps, useEditStat } from "../hooks/useEditStat";
import { Input } from "../forms/Input";
import { Check } from "../icons/Check";
import styles from "./MovementData.module.css";

const EditableStat: Component<EditableStatProps> = (props) => {
  const { amount, onSubmit, mutation, reset } = useEditStat(props);

  createEffect(() => {
    if (mutation.isSuccess) {
      const timeout = setTimeout(mutation.reset, 2000);
      onCleanup(() => clearTimeout(timeout));
    }
  });

  return (
    <form
      class="flex flex-row items-center gap-1"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <div class="flex-shrink-0">{props.type === "max" ? "Max:" : "Reps:"}</div>
      <div class="flex-grow">
        <Input
          control={amount}
          type="number"
          min={0}
          class="w-full h-full ghost-input"
          placeholder="Edit"
          disabled={mutation.isLoading}
          required={props.type === "max"}
          onBlur={reset}
        />
      </div>
      <Show when={mutation.isSuccess}>
        <Check
          class="text-green-500 flex-shrink-0 ml-2"
          classList={{
            [styles["fade-out"]!]: true,
          }}
        />
      </Show>
    </form>
  );
};

export const MovementData: Component<{
  maxes: Max[];
  reps: Reps[];
  movement: Movement;
  profileId: string;
}> = (props) => {
  const [showForm, setShowForm] = createSignal<"max" | "reps" | null>(null);

  return (
    <div class="flex flex-col gap-2">
      <div class="grid grid-cols-5 gap-1">
        <div class="col-span-2">
          <EditableStat
            profileId={props.profileId}
            type="max"
            movement={props.movement}
            stat={props.maxes[props.maxes.length - 1]}
          />
          <EditableStat
            profileId={props.profileId}
            type="reps"
            movement={props.movement}
            stat={props.reps[props.reps.length - 1]}
          />
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
