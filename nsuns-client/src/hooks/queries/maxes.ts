import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import { CreateMax, Max, createMax, getMaxes } from "../../api/maxes";

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
