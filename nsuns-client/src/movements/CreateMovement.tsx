import { createFormControl, createFormGroup } from "solid-forms";
import { Component } from "solid-js";
import styles from "./CreateMovement.module.css";
import { TextInput } from "../forms/TextInput";
import { TextArea } from "../forms/Textarea";
import { hasErrors } from "../forms/errors";
import { useCreateMovement } from "../hooks/queries/movements";

export const CreateMovement: Component<{ cancel: () => void }> = (props) => {
  const group = createFormGroup({
    name: createFormControl("", { required: true }),
    description: createFormControl(""),
  });
  const mutation = useCreateMovement({
    onSuccess: () => {
      props.cancel();
    },
  });

  const anyErrors = () => {
    return !Object.values(group.controls).every(
      (control) => !hasErrors(control.errors)
    );
  };

  const onSubmit = () => {
    if (mutation.isLoading || anyErrors()) return;
    mutation.mutate({
      name: group.value.name || "",
      description: group.value.description || null,
    });
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
      classList={{ [styles.form]: true }}
    >
      <label for="movement-name">
        <span class="text-red-500">*</span>Name
      </label>
      <TextInput
        control={group.controls.name}
        class="input w-full my-2"
        name="movement-name"
      />
      <label for="movement-description">Description</label>
      <TextArea
        control={group.controls.description}
        class="input w-full my-2"
        name="movement-description"
      />
      <div class="flex flex-row items-center mt-2">
        <button
          type="button"
          class="secondary-button mr-2"
          disabled={mutation.isLoading}
          onClick={props.cancel}
        >
          Cancel
        </button>
        <button
          type="submit"
          class="primary-button"
          disabled={mutation.isLoading || anyErrors()}
        >
          Create Movement
        </button>
      </div>
    </form>
  );
};
