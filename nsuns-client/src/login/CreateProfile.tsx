import { Component, Show } from "solid-js";
import { Input } from "../forms/Input";
import { A } from "@solidjs/router";
import { createProfile } from "../api";
import styles from "./CreateProfile.module.css";
import { createMutation, useQueryClient } from "@tanstack/solid-query";
import { Spinner } from "../icons/Spinner";
import { useNavigateToProfileHome } from "../hooks/navigation";
import { createControl } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";

export const CreateProfile: Component = () => {
  const queryClient = useQueryClient();
  const navigateToProfileHome = useNavigateToProfileHome();

  const mutation = createMutation({
    mutationFn: createProfile,
    onSuccess: (profile) => {
      queryClient.invalidateQueries(["profiles"], {
        exact: false,
      });
      navigateToProfileHome(profile.id);
    },
  });

  const name = createControl("");

  const onSubmit = async () => {
    if (mutation.isLoading || name.hasErrors()) return;

    mutation.mutate({
      name: name.value(),
    });
  };

  return (
    <div class="w-full h-full flex flex-col items-center justify-center">
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          await onSubmit();
        }}
        class="mx-3 grid gap-y-2 gap-x-4"
        classList={{ [styles.form]: true }}
      >
        <h2 class="col-span-2 text-lg mb-4">Create Profile</h2>
        <label for="name" class="label-left">
          <span class="text-red-500">*</span>Name
        </label>
        <div class="flex flex-col items-end">
          <Input
            control={name}
            class="ml-3 input"
            name="name"
            required={true}
          />
          <ErrorMessages control={name} />
        </div>
        <div class="col-span-2 mt-4">
          <div class="float-right flex flex-row items-center justify-end w-full">
            <A href="/" class="text-button text-center mr-2">
              Home
            </A>
            <button
              type="button"
              onClick={() => name.reset()}
              class="secondary-button mr-2"
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
          <Show when={mutation.isError}>{`${mutation.error}`}</Show>
        </div>
      </form>
    </div>
  );
};
