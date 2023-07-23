import {
  CreateMutationOptions,
  createMutation,
  useQueryClient,
} from "@tanstack/solid-query";
import { UpdateRequest, UpdateResponse, runUpdates } from "../../api/updates";
import { QueryKeys } from "./keys";
import { RepsQueryData } from "./reps";
import { MaxesQueryData } from "./maxes";

export const createRunUpdatesMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<UpdateResponse, TError, UpdateRequest, TContext>
  >
) => {
  const queryClient = useQueryClient();

  return createMutation({
    mutationFn: runUpdates,
    ...options,
    onSuccess: (updates, payload, ...args) => {
      options?.onSuccess?.(updates, payload, ...args);

      queryClient.setQueryData(
        QueryKeys.reps(payload.profileId),
        (reps?: RepsQueryData) => reps && reps.concat(updates.reps)
      );

      queryClient.setQueryData(
        QueryKeys.maxes(payload.profileId),
        (maxes?: MaxesQueryData) => maxes && maxes.concat(updates.maxes)
      );
    },
  });
};
