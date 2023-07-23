import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateMax,
  Max,
  UpdateMax,
  createMax,
  getMaxes,
  updateMax,
} from "../../api/maxes";
import { Accessor } from "solid-js";
import { updateInArray } from "./util";
import { QueryKeys } from "./keys";

export const useMaxesQuery = (profileId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.maxes(profileId()),
    queryFn: () => getMaxes(profileId()),
    enabled: !!profileId(),
  });
};

export const useCreateMaxMutation = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<CreateMutationOptions<Max, TError, CreateMax, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createMax,
    onSuccess: (max, ...args) => {
      options?.onSuccess?.(max, ...args);
      queryClient.setQueryData(QueryKeys.maxes(profileId()), (maxes?: Max[]) => {
        return maxes && [...maxes, max];
      });
    },
  });
  return mutation;
};

export const useUpdateMaxMutation = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<CreateMutationOptions<Max, TError, UpdateMax, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateMax,
    onSuccess: (max, ...args) => {
      options?.onSuccess?.(max, ...args);
      queryClient.setQueryData(QueryKeys.maxes(profileId()), (maxes?: Max[]) =>
        updateInArray(maxes, max, (m) => m.id === max.id)
      );
    },
  });
  return mutation;
};
