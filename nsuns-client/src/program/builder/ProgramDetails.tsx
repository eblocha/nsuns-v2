import { Component, Show, createRenderEffect, on } from "solid-js";
import { Input } from "../../forms/Input";
import { Program } from "../../api";
import { Spinner } from "../../icons/Spinner";
import { createDelayedLatch } from "../../hooks/createDelayedLatch";
import { useUpdateProgram } from "../../hooks/queries/programs";
import { createControl, required } from "../../hooks/forms";
import { ErrorMessages } from "../../forms/ErrorMessages";

export const ProgramDetails: Component<Program> = (props) => {
  const name = createControl(props.name, { validators: [required()] });

  const mutation = useUpdateProgram();

  const onSubmit = async () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      id: props.id,
      name: name.value(),
    });
  };

  createRenderEffect(
    on(
      () => props.name,
      () => {
        name.reset(props.name);
      }
    )
  );

  const isLoading = createDelayedLatch(() => mutation.isLoading, 200);

  return (
    <form
      class="flex flex-row items-center justify-between mb-2"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <div class="flex flex-col items-end">
        <Input control={name} class="ghost-input" required={true} />
        <ErrorMessages control={name} />
      </div>
      <button
        class="primary-button ml-4 w-16 flex flex-row items-center justify-center"
        disabled={isLoading() || name.hasErrors() || !name.dirty()}
      >
        <Show
          when={!isLoading()}
          fallback={<Spinner class="my-1 animate-spin" />}
        >
          Save
        </Show>
      </button>
    </form>
  );
};
