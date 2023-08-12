import { Component, Show } from "solid-js";
import { Input } from "../forms/Input";
import { A, useParams } from "@solidjs/router";
import { Spinner } from "../icons/Spinner";
import { useCreateProgram } from "../hooks/queries/programs";
import { useNavigateToProgram } from "../hooks/navigation";
import { createControl, required } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";

export const NewProgram: Component = () => {
  const params = useParams<{ profileId: string }>();
  const navigateToProgram = useNavigateToProgram();

  const name = createControl<string>("", { validators: [required()] });

  const mutation = useCreateProgram({
    onSuccess: (program) => {
      navigateToProgram(program.id);
    },
  });

  const onSubmit = () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      name: name.value(),
      owner: params.profileId,
      description: null,
    });
  };

  return (
    <div class="w-full h-full p-14 border-l border-gray-700 flex flex-col items-start justify-center gap-4">
      <h2 class="text-2xl">Create New Program</h2>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit();
        }}
        class="flex flex-col w-80 gap-4"
      >
        <label
          for="program-name"
          class="flex flex-row items-center gap-2"
        >
          <div>
            <span class="text-red-500">*</span>Title
          </div>
          <div class="flex flex-col items-end flex-grow">
            <Input
              control={name}
              class="input w-full"
              name="program-name"
              required={true}
            />
            <ErrorMessages control={name} />
          </div>
        </label>

        <div class="flex flex-row items-center justify-end">
          <A
            href="../.."
            class="text-button"
          >
            Cancel
          </A>
          <button
            class="primary-button ml-2 flex flex-row items-center justify-center w-36"
            disabled={mutation.isLoading || name.hasErrors()}
          >
            <Show
              when={!mutation.isLoading}
              fallback={<Spinner class="animate-spin my-1" />}
            >
              Create Program
            </Show>
          </button>
        </div>
        <Show when={mutation.isError}>
          <div class="w-full flex flex-row items-center justify-end gap-4">
            <span>
              <Warning class="text-red-500" />
            </span>
            {displayError(mutation.error, "create program")}
          </div>
        </Show>
      </form>
    </div>
  );
};
