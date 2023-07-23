import { Component, Show, createSignal } from "solid-js";
import { Plus } from "../../../icons/Plus";
import { NewMaxForm } from "./NewMaxForm";
import { useStats } from "../../../stats/StatsProvider";

export const AddMax: Component = () => {
  const { profileId } = useStats();

  const [showForm, setShowForm] = createSignal();

  return (
    <Show
      when={!showForm()}
      fallback={
        <NewMaxForm close={() => setShowForm(false)} profileId={profileId()} />
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
