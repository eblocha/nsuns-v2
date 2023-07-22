import { Component } from "solid-js";
import { Movement } from "../api";
import { createControl, createControlGroup, required } from "../hooks/forms";
import { useUpdateMovement } from "../hooks/queries/movements";
import { MovementForm } from "./MovementForm";

export const UpdateMovement: Component<{
  cancel: () => void;
  movement: Movement;
}> = (props) => {
  const group = createControlGroup({
    name: createControl(props.movement.name, { validators: [required()] }),
    description: createControl(props.movement.description ?? ""),
  });
  const mutation = useUpdateMovement({
    onSuccess: () => {
      props.cancel();
    },
  });

  return (
    <MovementForm
      confirmText="Save Movement"
      group={group}
      onClose={props.cancel}
      mutationUpdate={mutation}
      id={props.movement.id}
    />
  );
};
