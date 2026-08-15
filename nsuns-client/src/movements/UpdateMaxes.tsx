import { Component, Show } from "solid-js";
import { createRunUpdatesMutation } from "../hooks/queries/updates";
import { Rotate } from "../icons/Rotate";

export const UpdateMaxes: Component<{
  movementId: string;
  profileId: string;
}> = (props) => {
  const mutation = createRunUpdatesMutation();

  const run = () => {
    if (mutation.isPending) return;
    mutation.mutate({
      movementIds: [props.movementId],
      profileId: props.profileId,
    });
  };

  return (
    <button
      class="text-button flex flex-row items-center gap-2"
      onClick={run}
      disabled={mutation.isPending}
    >
      <Rotate
        classList={{
          "animate-spin": mutation.isPending,
        }}
      />
      <Show
        when={mutation.isPending}
        fallback={"Run Updates"}
      >
        Updating...
      </Show>
    </button>
  );
};
