import { Component, Show, createSignal } from "solid-js";
import { Plus } from "../../icons/Plus";
import { NewMaxForm } from "./NewMaxForm";

export const AddMax: Component<{ profileId: string }> = (props) => {
  const [showForm, setShowForm] = createSignal();

  return (
    <Show
      when={!showForm()}
      fallback={
        <NewMaxForm
          close={() => setShowForm(false)}
          profileId={props.profileId}
        />
      }
    >
      <button
        class="flex flex-row items-center gap-2 text-button-outline"
        onClick={() => setShowForm(true)}
      >
        <Plus /> Log a New Max
      </button>
    </Show>
  );
};
