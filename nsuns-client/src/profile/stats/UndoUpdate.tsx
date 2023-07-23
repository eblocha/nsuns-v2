import { Component, Show } from "solid-js";
import { useStats } from "../../stats/StatsProvider";
import { createUndoUpdatesMutation } from "../../hooks/queries/updates";
import { RotateLeft } from "../../icons/RotateLeft";

export const UndoUpdate: Component = () => {
  const { profileId, movementMap } = useStats();

  const mutation = createUndoUpdatesMutation();

  const run = () => {
    if (mutation.isLoading) return;
    mutation.mutate({
      movementIds: Object.values(movementMap()).map((mv) => mv.id),
      profileId: profileId(),
    });
  };

  return (
    <button
      class="text-button flex flex-row items-center gap-2"
      onClick={run}
      disabled={mutation.isLoading}
    >
      <RotateLeft />
      <Show when={mutation.isLoading} fallback={"Undo Update"}>
        Undoing...
      </Show>
    </button>
  );
};
