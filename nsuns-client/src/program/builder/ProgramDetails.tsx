import { Component, Show, createRenderEffect } from "solid-js";
import { Input } from "../../forms/Input";
import { Program } from "../../api";
import { Spinner } from "../../icons/Spinner";
import { createSmartAsyncDelay } from "../../hooks/asymmetricDelay";
import { useUpdateProgram } from "../../hooks/queries/programs";
import { createControl, required } from "../../hooks/forms";
import { Warning } from "../../icons/Warning";
import { displayError } from "../../util/errors";

export const ProgramDetails: Component<{
  program: Program;
  profileId: string;
}> = (props) => {
  const name = createControl(props.program.name, { validators: [required()] });
  const reset = () => name.reset(props.program.name);

  const mutation = useUpdateProgram({
    onSuccess: (program) => {
      name.reset(program.name);
    },
  });

  const onSubmit = () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      id: props.program.id,
      name: name.value(),
      description: null,
    });
  };

  createRenderEffect(reset);

  const isLoading = createSmartAsyncDelay(() => mutation.isLoading);

  return (
    <form
      class="flex flex-col"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <div class="flex flex-row items-center">
        <div class="flex flex-col items-end text-2xl">
          <Input
            control={name}
            class="ghost-input"
            required={true}
          />
        </div>
        <Show when={name.isChanged()}>
          <button
            onClick={reset}
            class="secondary-button ml-auto"
            type="button"
          >
            Reset Title
          </button>
          <button
            class="primary-button ml-4 w-16 flex flex-row items-center justify-center"
            disabled={isLoading() || name.hasErrors()}
            type="submit"
          >
            <Show
              when={!isLoading()}
              fallback={<Spinner class="my-1 animate-spin" />}
            >
              Save
            </Show>
          </button>
        </Show>
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
