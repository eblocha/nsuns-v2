import { Component, Show, createSignal, mergeProps } from "solid-js";
import { Edit } from "../icons/Edit";
import { UpdateMovement } from "./UpdateMovement";
import { Movement } from "../api";
import { Max } from "../api/maxes";
import { Reps } from "../api/reps";
import { ChevronDown } from "../icons/ChevronDown";
import { MovementData } from "./MovementData";
import { plural } from "../util/setDisplay";

const maxSummary = (amount?: number) => (amount === undefined ? "" : `max ${amount} lb${plural(amount)}`);

export const MovementItem: Component<{
  movement: Movement;
  profileId: string;
  maxes?: Max[];
  reps?: Reps[];
}> = (props) => {
  const mergedProps = mergeProps({ maxes: [] as Max[], reps: [] as Reps[] }, props);

  const [showForm, setShowForm] = createSignal(false);
  const [isExpanded, setIsExpanded] = createSignal(false);

  return (
    <li class="border rounded border-gray-400 p-2 my-1 flex flex-col gap-4">
      <Show
        when={!showForm()}
        fallback={
          <UpdateMovement
            cancel={() => setShowForm(false)}
            movement={{
              id: props.movement.id,
              name: props.movement.name,
              description: props.movement.description,
            }}
          />
        }
      >
        <div class="flex flex-row items-center gap-1">
          <div>
            <p>
              {props.movement.name}
              <Show when={!isExpanded()}>
                <span class="italic text-gray-400">
                  {" " + maxSummary(mergedProps.maxes[mergedProps.maxes.length - 1]?.amount)}
                </span>
              </Show>
            </p>
            <p class="text-sm opacity-80">{props.movement.description}</p>
          </div>
          <button
            class="text-button ml-auto"
            onClick={() => setShowForm(true)}
          >
            <Edit class="text-gray-300" />
          </button>
          <button
            class="text-button"
            onClick={() => setIsExpanded((e) => !e)}
          >
            <ChevronDown
              classList={{
                "rotate-180": isExpanded(),
              }}
            />
          </button>
        </div>
        <Show when={isExpanded()}>
          <MovementData
            maxes={mergedProps.maxes}
            reps={mergedProps.reps}
            movement={props.movement}
            profileId={props.profileId}
          />
        </Show>
      </Show>
    </li>
  );
};
