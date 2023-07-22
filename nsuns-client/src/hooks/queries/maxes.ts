import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateMax,
  Max,
  createMax,
  getMaxes,
  updateMax,
} from "../../api/maxes";

export const useMaxesQuery = (profileId: string) => {
  return createQuery({
    queryKey: () => ["maxes", profileId],
    queryFn: () => getMaxes(profileId),
    enabled: !!profileId,
  });
};

export const useCreateMaxMutation = <TError = unknown, TContext = unknown>(
  profileId: string,
  options?: Partial<CreateMutationOptions<Max, TError, CreateMax, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createMax,
    onSuccess: (max, ...args) => {
      queryClient.setQueryData(["maxes", profileId], (maxes?: Max[]) => {
        return maxes && [...maxes, max];
      });
      options?.onSuccess?.(max, ...args);
    },
  });
  return mutation;
};

export const useUpdateMaxMutation = <TError = unknown, TContext = unknown>(
  profileId: string,
  options?: Partial<CreateMutationOptions<Max, TError, Max, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateMax,
    onSuccess: (max, ...args) => {
      queryClient.setQueryData(["maxes", profileId], (maxes?: Max[]) => {
        if (!maxes) return maxes;

        const index = maxes.findIndex((m) => m.id === max.id);
        if (index === -1) return maxes;

        const newMaxes = [...maxes];
        newMaxes.splice(index, 1, max);

        return newMaxes;
      });
      options?.onSuccess?.(max, ...args);
    },
  });
  return mutation;
};
