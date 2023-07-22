import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateProgram,
  Program,
  UpdateProgram,
  createProgram,
  deleteProgram,
  getProfilePrograms,
  updateProgram,
} from "../../api";
import { Accessor } from "solid-js";
import { updateInArray } from "./util";

export const useProgramsQuery = (profileId: string) => {
  const programsQuery = createQuery({
    queryKey: () => ["programs", profileId],
    queryFn: () => getProfilePrograms(profileId),
    enabled: !!profileId,
  });
  return programsQuery;
};

export const useCreateProgram = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<
    CreateMutationOptions<Program, TError, CreateProgram, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createProgram,
    onSuccess: (program, ...args) => {
      options?.onSuccess?.(program, ...args);
      queryClient.setQueryData(
        ["programs", profileId()],
        (programs?: Program[]) =>
          programs ? [...programs, program] : undefined
      );
    },
  });
  return mutation;
};

export const useUpdateProgram = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<
    CreateMutationOptions<Program, TError, UpdateProgram, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateProgram,
    onSuccess: (program, ...args) => {
      options?.onSuccess?.(program, ...args);
      queryClient.setQueryData(
        ["programs", profileId()],
        (programs?: Program[]) =>
          updateInArray(programs, program, (p) => p.id === program.id)
      );
    },
  });
  return mutation;
};

export const useDeleteProgram = <TError = unknown, TContext = unknown>(
  profileId: Accessor<string>,
  options?: Partial<
    CreateMutationOptions<void, TError, string | number, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: deleteProgram,
    onSuccess: (program, id, ...args) => {
      options?.onSuccess?.(program, id, ...args);
      queryClient.setQueryData(
        ["programs", profileId()],
        (programs?: Program[]) => programs?.filter((p) => p.id !== id)
      );
    },
  });
  return mutation;
};
