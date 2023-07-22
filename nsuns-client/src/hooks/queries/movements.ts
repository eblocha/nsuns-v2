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
  updateMovement,
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
    onSuccess: (movement, ...args) => {
      queryClient.setQueryData(["movements"], (movements?: Movement[]) => {
        return movements && [...movements, movement];
      });
      options?.onSuccess?.(movement, ...args);
    },
  });
  return mutation;
};

export const useUpdateMovement = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Movement, TError, Movement, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateMovement,
    onSuccess: (movement, ...args) => {
      queryClient.setQueryData(["movements"], (movements?: Movement[]) => {
        if (!movements) return;

        const index = movements.findIndex((mv) => mv.id === movement.id);

        if (index === -1) return;

        const newMovements = [...movements];

        newMovements.splice(index, 1, movement);

        return newMovements;
      });
      options?.onSuccess?.(movement, ...args);
    },
  });
  return mutation;
};
