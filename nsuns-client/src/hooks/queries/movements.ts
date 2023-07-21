import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateMovement,
  Movement,
  createMovement,
  getMovements,
} from "../../api";

export const useMovementsQuery = () => {
  const query = createQuery({
    queryKey: () => ["movements"],
    queryFn: getMovements,
  });

  return query;
};

export const useCreateMovement = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<Movement, TError, CreateMovement, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createMovement,
    onSuccess: (...args) => {
      queryClient.invalidateQueries(["movements"]);
      options?.onSuccess?.(...args);
    },
  });
  return mutation;
};
