import { MutationOptions, useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
import { CreateMovement, Movement, createMovement, getMovements, updateMovement } from "../../api";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";

export type MovementsQueryData = QueryData<ReturnType<typeof useMovementsQuery>>;

export const useMovementsQuery = () => {
  const query = useQuery(() => ({
    queryKey: QueryKeys.movements(),
    queryFn: getMovements,
    staleTime: Infinity,
  }));

  return query;
};

export const useCreateMovement = <TError = unknown, TContext = unknown>(
  options?: Partial<MutationOptions<Movement, TError, CreateMovement, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = useMutation(() => ({
    ...options,
    mutationFn: createMovement,
    onSuccess: (movement, ...args) => {
      queryClient.setQueryData(QueryKeys.movements(), (movements?: MovementsQueryData) => {
        return movements && [...movements, movement];
      });
      options?.onSuccess?.(movement, ...args);
    },
  }));
  return mutation;
};

export const useUpdateMovement = <TError = unknown, TContext = unknown>(
  options?: Partial<MutationOptions<Movement, TError, Movement, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = useMutation(() => ({
    ...options,
    mutationFn: updateMovement,
    onSuccess: (movement, ...args) => {
      options?.onSuccess?.(movement, ...args);
      queryClient.setQueryData(QueryKeys.movements(), (movements?: MovementsQueryData) =>
        updateInArray(movements, movement, (m) => m.id === movement.id)
      );
    },
  }));
  return mutation;
};
