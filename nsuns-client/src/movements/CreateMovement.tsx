import { Component } from "solid-js";
import styles from "./CreateMovement.module.css";
import { Input } from "../forms/Input";
import { TextArea } from "../forms/TextArea";
import { useCreateMovement } from "../hooks/queries/movements";
import { createControl, createControlGroup, required } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";

export const CreateMovement: Component<{ cancel: () => void }> = (props) => {
  const group = createControlGroup({
    name: createControl("", { validators: [required()] }),
    description: createControl(""),
  });
  const mutation = useCreateMovement({
    onSuccess: () => {
      props.cancel();
    },
  });

  const onSubmit = () => {
    if (mutation.isLoading || group.hasErrors()) return;
    const value = group.value();

    mutation.mutate({
      name: value.name,
      description: value.description || null,
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
      <div class="flex flex-col items-end">
        <Input
          control={group.controls.name}
          class="input w-full my-2"
          name="movement-name"
          required={true}
        />
        <ErrorMessages control={group.controls.name} />
      </div>
      <label for="movement-description">Description</label>
      <div class="flex flex-col items-end">
        <TextArea
          control={group.controls.description}
          class="input w-full my-2"
          name="movement-description"
        />
        <ErrorMessages control={group.controls.description} />
      </div>
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
          disabled={mutation.isLoading || group.hasErrors()}
        >
          Create Movement
        </button>
      </div>
    </form>
  );
};
