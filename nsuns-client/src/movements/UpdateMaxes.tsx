import { Component, Show } from "solid-js";
import { createRunUpdatesMutation } from "../hooks/queries/updates";
import { Rotate } from "../icons/Rotate";

export const UpdateMaxes: Component<{
  movementId: string;
  profileId: string;
}> = (props) => {
  const mutation = createRunUpdatesMutation();

  const run = () => {
    if (mutation.isLoading) return;
    mutation.mutate({
      movementIds: [props.movementId],
      profileId: props.profileId,
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
      <Show
        when={mutation.isLoading}
        fallback={"Run Updates"}
      >
        Updating...
      </Show>
    </button>
  );
};
