import { createFormControl, createFormGroup } from "solid-forms";
import { Component, Show } from "solid-js";
import { TextInput } from "../forms/TextInput";
import { A, useNavigate } from "@solidjs/router";
import { createProfile } from "../api";
import styles from "./CreateProfile.module.css";
import { hasErrors } from "../forms/errors";
import { createMutation, useQueryClient } from "@tanstack/solid-query";

export const CreateProfile: Component = () => {
  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationFn: createProfile,
    onSuccess: (profile) => {
      queryClient.invalidateQueries(["profiles"], {
        exact: false,
      });
      navigator(`/profile/${profile.id}`);
    },
  });

  const navigator = useNavigate();

  const group = createFormGroup({
    name: createFormControl("", {
      required: true,
    }),
  });

  const onSubmit = async () => {
    if (mutation.isLoading || anyErrors()) return;

    mutation.mutate({
      name: group.value.name!,
    });
  };

  const resetForm = () => {
    for (const control of Object.values(group.controls)) {
      control.markDirty(false);
      control.markTouched(false);
      control.setValue("");
    }
  };

  const anyErrors = () => {
    return !Object.values(group.controls).every(
      (control) => !hasErrors(control.errors)
    );
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
        <TextInput
          control={group.controls.name}
          class="ml-3 input"
          name="name"
        />
        <div class="col-span-2 mt-4">
          <div class="float-right flex flex-row items-center justify-end w-full">
            <A href="/" class="text-button text-center mr-2">
              Home
            </A>
            <button
              type="button"
              onClick={resetForm}
              class="secondary-button mr-2"
              disabled={!group.isDirty || mutation.isLoading}
            >
              Reset
            </button>
            <button
              type="submit"
              class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600 disabled:bg-blue-400"
              disabled={anyErrors() || mutation.isLoading}
            >
              <Show when={!mutation.isLoading} fallback={<>Creating...</>}>
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
