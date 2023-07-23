import { Component } from "solid-js";
import { Day, Movement, ProgramSet } from "../../api";
import { useDeleteSet, useEditSet } from "../../hooks/queries/sets";
import { createControl, createControlGroup, required } from "../../hooks/forms";
import { SetForm } from "./SetForm";

export const EditSet: Component<{
  close: () => void;
  set: ProgramSet;
  dayIndex: Day;
  programId: number;
  movements?: Movement[];
}> = (props) => {
  const mutation = useEditSet({
    onSuccess: props.close,
  });

  const deleteMutation = useDeleteSet({
    onSuccess: props.close,
  });

  const group = createControlGroup({
    movementId: createControl<string>(props.set.movementId.toString(), {
      validators: [required()],
    }),
    reps: createControl<string>(props.set.reps?.toString() ?? ""),
    repsIsMinimum: createControl(props.set.repsIsMinimum),
    description: createControl<string>(props.set.description ?? ""),
    amount: createControl<string>(props.set.amount.toString(), {
      validators: [required()],
    }),
    maxMovementId: createControl<string>(
      props.set.percentageOfMax?.toString() ?? ""
    ),
  });

  return (
    <div class="p-4 rounded">
      <SetForm
        group={group}
        dayIndex={props.dayIndex}
        mutationUpdate={mutation}
        mutationDelete={deleteMutation}
        id={props.set.id}
        onClose={props.close}
        programId={props.programId}
        title="Edit Set"
        movements={props.movements}
      />
    </div>
  );
};
