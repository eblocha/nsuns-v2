import { Component, Show } from "solid-js";
import { Input } from "../forms/Input";
import { A } from "@solidjs/router";
import { Spinner } from "../icons/Spinner";
import { useNavigateToProfileHome } from "../hooks/navigation";
import { createControl, required } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";
import { createCreateProfileMutation } from "../hooks/queries/profiles";

export const CreateProfile: Component = () => {
  const navigateToProfileHome = useNavigateToProfileHome();

  const mutation = createCreateProfileMutation({
    onSuccess: (profile) => {
      navigateToProfileHome(profile.id);
    },
  });

  const name = createControl<string>("", { validators: [required()] });

  const onSubmit = () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      name: name.value(),
    });
  };

  return (
    <div class="w-full h-full flex flex-col items-center justify-center">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit();
        }}
        class="flex flex-col w-80 gap-4"
      >
        <h2 class="text-lg">Create Profile</h2>
        <label
          for="name"
          class="flex flex-row items-center gap-2"
        >
          <span class="text-red-500">*</span>Name
          <div class="flex flex-col items-end flex-grow">
            <Input
              control={name}
              class="ml-3 input w-full"
              name="name"
              required={true}
            />
            <ErrorMessages control={name} />
          </div>
        </label>

        <div class="float-right flex flex-row items-center justify-end w-full gap-2">
          <A
            href="/"
            class="text-button text-center"
          >
            Home
          </A>
          <button
            type="button"
            onClick={() => name.reset()}
            class="secondary-button"
            disabled={!name.dirty() || mutation.isLoading}
          >
            Reset
          </button>
          <button
            type="submit"
            class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600 disabled:bg-blue-400 w-32 flex flex-row items-center justify-center"
            disabled={mutation.isLoading || name.hasErrors()}
          >
            <Show
              when={!mutation.isLoading}
              fallback={<Spinner class="animate-spin my-1" />}
            >
              Create Profile
            </Show>
          </button>
        </div>
        <Show when={mutation.isError}>
          <div class="w-full flex flex-row items-center justify-end gap-4">
            <span>
              <Warning class="text-red-500" />
            </span>
            {displayError(mutation.error, "create profile")}
          </div>
        </Show>
      </form>
    </div>
  );
};
