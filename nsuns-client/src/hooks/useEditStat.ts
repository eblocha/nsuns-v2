import { createRenderEffect } from "solid-js";
import { Movement } from "../api";
import { Max } from "../api/maxes";
import { Reps } from "../api/reps";
import { createControl } from "./forms";
import { useCreateMaxMutation, useUpdateMaxMutation } from "./queries/maxes";
import { useCreateRepsMutation, useUpdateRepsMutation } from "./queries/reps";
import { createMutation } from "@tanstack/solid-query";

export type CommonProps = {
  movement?: Movement;
  profileId: string;
};

export type EditableStatProps = CommonProps &
  (
    | {
        stat?: Max;
        type: "max";
      }
    | {
        stat?: Reps;
        type: "reps";
      }
  );

export const useEditStat = (props: EditableStatProps) => {
  const amount = createControl(props.stat?.amount?.toString() || "");

  const reset = () => {
    amount.reset(props.stat?.amount?.toString() ?? "");
  };

  createRenderEffect(reset);

  const options = {
    onError: reset,
  };

  const updateMax = useUpdateMaxMutation(options);
  const createMax = useCreateMaxMutation(options);

  const updateReps = useUpdateRepsMutation(options);
  const createReps = useCreateRepsMutation(options);

  const mutation = createMutation({
    mutationFn: async ({ amount, movement }: { amount: number | null; movement: Movement }) => {
      if (props.stat && props.type === "max" && amount !== null) {
        await updateMax.mutateAsync({
          id: props.stat.id,
          amount,
        });
      } else if (props.stat && props.type === "reps") {
        await updateReps.mutateAsync({
          id: props.stat.id,
          amount,
        });
      } else if (props.type === "max" && amount !== null) {
        await createMax.mutateAsync({
          amount,
          movementId: movement.id,
          profileId: props.profileId,
        });
      } else if (props.type === "reps") {
        await createReps.mutateAsync({
          amount,
          movementId: movement.id,
          profileId: props.profileId,
        });
      }
    },
  });

  const onSubmit = () => {
    const amt = amount.value();
    if (mutation.isLoading || !props.movement) return;

    const parsed = amt ? parseInt(amt) : null;

    if (parsed === props.stat?.amount) return;

    mutation.mutate({ amount: parsed, movement: props.movement });
  };

  return { onSubmit, amount, reset, mutation };
};
