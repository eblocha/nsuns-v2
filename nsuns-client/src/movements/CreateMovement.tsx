import { Component } from "solid-js";
import { useCreateMovement } from "../hooks/queries/movements";
import { createControl, createControlGroup, required } from "../hooks/forms";
import { MovementForm } from "./MovementForm";

export const CreateMovement: Component<{ cancel: () => void }> = (props) => {
  const group = createControlGroup({
    name: createControl<string>("", { validators: [required()] }),
    description: createControl(""),
  });
  const mutation = useCreateMovement({
    onSuccess: () => {
      props.cancel();
    },
  });

  return (
    <MovementForm
      confirmText="Create Movement"
      group={group}
      onClose={props.cancel}
      mutationCreate={mutation}
    />
  );
};
