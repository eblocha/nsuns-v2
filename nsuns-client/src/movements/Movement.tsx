import { Component, Show, createSignal } from "solid-js";
import { Edit } from "../icons/Edit";
import { UpdateMovement } from "./UpdateMovement";

export const Movement: Component<{
  id: number;
  name: string;
  description: string | null;
}> = (props) => {
  const [showForm, setShowForm] = createSignal(false);

  return (
    <li class="border rounded border-gray-400 p-2 my-1 flex flex-row items-center justify-between">
      <Show
        when={!showForm()}
        fallback={
          <UpdateMovement
            cancel={() => setShowForm(false)}
            movement={{
              id: props.id,
              name: props.name,
              description: props.description,
            }}
          />
        }
      >
        <div>
          <p>{props.name}</p>
          <p class="text-sm opacity-80">{props.description}</p>
        </div>
        <button class="text-button" onClick={() => setShowForm(true)}>
          <Edit class="text-gray-300" />
        </button>
      </Show>
    </li>
  );
};
