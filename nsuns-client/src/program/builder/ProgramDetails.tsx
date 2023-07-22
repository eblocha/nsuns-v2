import { Component, Show, createRenderEffect, on } from "solid-js";
import { Input } from "../../forms/Input";
import { Program } from "../../api";
import { Spinner } from "../../icons/Spinner";
import { createDelayedLatch } from "../../hooks/createDelayedLatch";
import { useUpdateProgram } from "../../hooks/queries/programs";
import { createControl, required } from "../../hooks/forms";
import { ErrorMessages } from "../../forms/ErrorMessages";
import { Warning } from "../../icons/Warning";
import { displayError } from "../../util/errors";

export const ProgramDetails: Component<{
  program: Program;
  profileId: string;
}> = (props) => {
  const name = createControl(props.program.name, { validators: [required()] });

  const mutation = useUpdateProgram(() => props.profileId, {
    onSuccess: () => {
      name.setDirty(false);
      name.setTouched(false);
    },
  });

  const onSubmit = async () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      id: props.program.id,
      name: name.value(),
    });
  };

  createRenderEffect(() => {
    name.reset(props.program.name);
  });

  const isLoading = createDelayedLatch(() => mutation.isLoading, 200);

  return (
    <form
      class="flex flex-col mb-2"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <div class="flex flex-row items-center justify-between">
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
      </div>
      <Show when={mutation.isError}>
        <div class="w-full flex flex-row items-center justify-end gap-4 mt-2">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(mutation.error, "update name")}
        </div>
      </Show>
    </form>
  );
};
