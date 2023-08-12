import { CreateMutationOptions, createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { CreateMax, Max, UpdateMax, createMax, getMaxes, updateMax } from "../../api/maxes";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";

export type MaxesQueryData = QueryData<ReturnType<typeof useMaxesQuery>>;

export const useMaxesQuery = (profileId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.maxes(profileId()),
    queryFn: () => getMaxes(profileId()),
    enabled: !!profileId(),
  });
};

export const useCreateMaxMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Max, TError, CreateMax, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createMax,
    onSuccess: (max, ...args) => {
      options?.onSuccess?.(max, ...args);
      queryClient.setQueryData(QueryKeys.maxes(max.profileId), (maxes?: MaxesQueryData) => {
        return maxes && [...maxes, max];
      });
    },
  });
  return mutation;
};

export const useUpdateMaxMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Max, TError, UpdateMax, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateMax,
    onSuccess: (max, ...args) => {
      options?.onSuccess?.(max, ...args);
      queryClient.setQueryData(QueryKeys.maxes(max.profileId), (maxes?: MaxesQueryData) =>
        updateInArray(maxes, max, (m) => m.id === max.id)
      );
    },
  });
  return mutation;
};
