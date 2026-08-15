import { Component, Show } from "solid-js";
import { createUndoUpdatesMutation } from "../hooks/queries/updates";
import { RotateLeft } from "../icons/RotateLeft";

export const UndoUpdate: Component<{
  movementId: string;
  profileId: string;
}> = (props) => {
  const mutation = createUndoUpdatesMutation();

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
      <RotateLeft />
      <Show
        when={mutation.isPending}
        fallback={"Undo Update"}
      >
        Undoing...
      </Show>
    </button>
  );
};
