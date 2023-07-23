import { Component, Show } from "solid-js";
import { Rotate } from "../../icons/Rotate";
import { createRunUpdatesMutation } from "../../hooks/queries/updates";
import { useStats } from "../../stats/StatsProvider";

export const UpdateMaxes: Component = () => {
  const { profileId, movementMap } = useStats();

  const mutation = createRunUpdatesMutation();

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
      <Rotate
        classList={{
          "animate-spin": mutation.isLoading,
        }}
      />
      <Show when={mutation.isLoading} fallback={"Update Maxes"}>
        Updating...
      </Show>
    </button>
  );
};
