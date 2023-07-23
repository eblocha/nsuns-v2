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
import { updateInArray } from "./util";
import { QueryKeys } from "./keys";

export const useMovementsQuery = () => {
  const query = createQuery({
    queryKey: QueryKeys.movements,
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
      queryClient.setQueryData(QueryKeys.movements(), (movements?: Movement[]) => {
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
      options?.onSuccess?.(movement, ...args);
      queryClient.setQueryData(QueryKeys.movements(), (movements?: Movement[]) =>
        updateInArray(movements, movement, (m) => m.id === movement.id)
      );
    },
  });
  return mutation;
};
