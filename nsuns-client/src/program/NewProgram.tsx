import { Component, Show } from "solid-js";
import { Input } from "../forms/Input";
import { useParams } from "@solidjs/router";
import { Spinner } from "../icons/Spinner";
import { useCreateProgram } from "../hooks/queries/programs";
import { useNavigateToProgram } from "../hooks/navigation";
import { createControl, required } from "../hooks/forms";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";

export const NewProgram: Component<{ close: () => void }> = (props) => {
  const params = useParams<{ profileId: string }>();
  const navigateToProgram = useNavigateToProgram();

  const name = createControl<string>("", { validators: [required()] });

  const mutation = useCreateProgram({
    onSuccess: (program) => {
      navigateToProgram(program.id);
      props.close();
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
    <div class="flex flex-col gap-2 p-3 border rounded border-gray-500">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit();
        }}
        class="flex flex-row gap-2 items-center"
      >
        <label for="program-name">
          <span class="text-red-500">*</span>Title
        </label>
        <Input
          control={name}
          class="input flex-grow"
          name="program-name"
          required={true}
          autofocus={true}
        />
        <button
          class="secondary-button"
          onClick={props.close}
          type="button"
        >
          Cancel
        </button>
        <button
          class="primary-button flex flex-row items-center justify-center w-20"
          disabled={mutation.isLoading || name.hasErrors()}
        >
          <Show
            when={!mutation.isLoading}
            fallback={<Spinner class="animate-spin my-1" />}
          >
            Create
          </Show>
        </button>
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
