import {
  CreateMutationOptions,
  createMutation,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  UndoResponse,
  UpdateRequest,
  UpdateResponse,
  runUpdates,
  undoUpdates,
} from "../../api/updates";
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

export const createUndoUpdatesMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<UndoResponse, TError, UpdateRequest, TContext>
  >
) => {
  const queryClient = useQueryClient();

  return createMutation({
    mutationFn: undoUpdates,
    ...options,
    onSuccess: (res, payload, ...args) => {
      options?.onSuccess?.(res, payload, ...args);

      queryClient.setQueryData(
        QueryKeys.reps(payload.profileId),
        (reps?: RepsQueryData) =>
          reps && reps.filter((r) => !res.reps.includes(r.id))
      );

      queryClient.setQueryData(
        QueryKeys.maxes(payload.profileId),
        (maxes?: MaxesQueryData) =>
          maxes && maxes.filter((m) => !res.maxes.includes(m.id))
      );
    },
  });
};
