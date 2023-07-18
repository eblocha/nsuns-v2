import { Component } from "solid-js";
import styles from "./NewProgram.module.css";
import { TextInput } from "../forms/TextInput";
import { createFormControl, createFormGroup } from "solid-forms";
import { TextArea } from "../forms/Textarea";
import { createMutation, useQueryClient } from "@tanstack/solid-query";
import { createProgram } from "../api";
import { A, useNavigate, useParams } from "@solidjs/router";
import { hasErrors } from "../forms/errors";

export const NewProgram: Component = () => {
  const params = useParams<{ userId: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const group = createFormGroup({
    name: createFormControl(""),
    description: createFormControl(""),
  });
  const mutation = createMutation({
    mutationFn: createProgram,
    onSuccess: (program) => {
      queryClient.invalidateQueries(["programs", params.userId]);
      navigate(`../${program.id}`);
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
      name: group.value.name || null,
      description: group.value.description || null,
      owner: params.userId,
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
          Title
        </label>
        <TextInput
          control={group.controls.name}
          class="ml-3 input"
          name="program-name"
        />
        <label for="program-description" class="label-left">
          Description
        </label>
        <TextArea
          control={group.controls.description}
          class="ml-3 input"
          name="program-description"
        />
        <div class="col-span-2 flex flex-row items-center justify-end mt-4">
          <A href="../.." class="text-button">
            Cancel
          </A>
          <button
            class="primary-button ml-2"
            disabled={mutation.isLoading || anyErrors()}
          >
            Create Program
          </button>
        </div>
      </form>
    </div>
  );
};
