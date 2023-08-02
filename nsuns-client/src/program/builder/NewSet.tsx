import { Component } from "solid-js";
import { Movement } from "../../api";
import { useCreateSet } from "../../hooks/queries/sets";
import { createControl, createControlGroup, required } from "../../hooks/forms";
import { SetForm } from "./SetForm";
import { Day } from "../../util/days";

export const NewSet: Component<{
  close: () => void;
  dayIndex: number;
  programId: string;
  movements?: Movement[];
}> = (props) => {
  const mutation = useCreateSet({
    onSuccess: () => {
      props.close();
    },
  });

  const group = createControlGroup({
    movementId: createControl<string>("", { validators: [required()] }),
    reps: createControl<string>(""),
    repsIsMinimum: createControl(false),
    description: createControl<string>(""),
    amount: createControl<string>("0", { validators: [required()] }),
    maxMovementId: createControl<string>(""),
  });

  return (
    <div class="border border-gray-700 p-4 rounded">
      <SetForm
        group={group}
        dayIndex={props.dayIndex as Day}
        mutationCreate={mutation}
        onClose={props.close}
        programId={props.programId}
        title="New Set"
        movements={props.movements}
      />
    </div>
  );
};
