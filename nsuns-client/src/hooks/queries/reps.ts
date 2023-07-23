import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import {
  CreateReps,
  Reps,
  UpdateReps,
  createReps,
  getReps,
  updateReps,
} from "../../api/reps";
import { QueryKeys } from "./keys";

export type RepsQueryData = QueryData<ReturnType<typeof useRepsQuery>>;

export const useRepsQuery = (profileId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.reps(profileId()),
    queryFn: () => getReps(profileId()),
    enabled: !!profileId(),
  });
};

export const useCreateRepsMutation = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<CreateMutationOptions<Reps, TError, CreateReps, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createReps,
    onSuccess: (reps, ...args) => {
      options?.onSuccess?.(reps, ...args);
      queryClient.setQueryData(QueryKeys.reps(profileId()), (repsList?: RepsQueryData) => {
        return repsList && [...repsList, reps];
      });
    },
  });
  return mutation;
};

export const useUpdateRepsMutation = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<CreateMutationOptions<Reps, TError, UpdateReps, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateReps,
    onSuccess: (reps, ...args) => {
      options?.onSuccess?.(reps, ...args);
      queryClient.setQueryData(QueryKeys.reps(profileId()), (repsList?: RepsQueryData) =>
        updateInArray(repsList, reps, (r) => r.id === reps.id)
      );
    },
  });
  return mutation;
};
