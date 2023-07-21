import { Component, Show } from "solid-js";
import styles from "./NewProgram.module.css";
import { TextInput } from "../forms/TextInput";
import { createFormControl, createFormGroup } from "solid-forms";
import { A, useParams } from "@solidjs/router";
import { hasErrors } from "../forms/errors";
import { Spinner } from "../icons/Spinner";
import { useCreateProgram } from "../hooks/queries/programs";
import { useNavigateToProgram } from "../hooks/navigation";

export const NewProgram: Component = () => {
  const params = useParams<{ profileId: string }>();
  const navigateToProgram = useNavigateToProgram();

  const group = createFormGroup({
    name: createFormControl("", { required: true }),
  });
  const mutation = useCreateProgram(params.profileId, {
    onSuccess: (program) => {
      navigateToProgram(program.id);
    },
  });

  const anyErrors = () => {
    return !Object.values(group.controls).every(
      (control) => !hasErrors(control.errors)
    );
  };

  const onSubmit = async () => {
    if (mutation.isLoading || anyErrors()) return;

    mutation.mutate({
      name: group.value.name!,
      owner: params.profileId,
    });
  };

  return (
    <div class="w-full h-full px-14 py-60 border-l border-gray-700">
      <h2 class="text-2xl">Create New Program</h2>
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          await onSubmit();
        }}
        class="grid gap-y-2 gap-x-4 mt-10"
        classList={{ [styles.form]: true }}
      >
        <label for="program-name" class="label-left">
          <span class="text-red-500">*</span>Title
        </label>
        <TextInput
          control={group.controls.name}
          class="ml-3 input"
          name="program-name"
        />
        <div class="col-span-2 flex flex-row items-center justify-end mt-4">
          <A href="../.." class="text-button">
            Cancel
          </A>
          <button
            class="primary-button ml-2 flex flex-row items-center justify-center w-36"
            disabled={mutation.isLoading || anyErrors()}
          >
            <Show
              when={!mutation.isLoading}
              fallback={<Spinner class="animate-spin my-1" />}
            >
              Create Program
            </Show>
          </button>
        </div>
      </form>
    </div>
  );
};
