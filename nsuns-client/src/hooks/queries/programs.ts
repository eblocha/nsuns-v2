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

export const useProgramsQuery = (profileId: string) => {
  const programsQuery = createQuery({
    queryKey: () => ["programs", profileId],
    queryFn: () => getProfilePrograms(profileId),
    enabled: !!profileId,
  });
  return programsQuery;
};

export const useCreateProgram = <TError = unknown, TContext = unknown>(
  profileId: string,
  options?: Partial<
    CreateMutationOptions<Program, TError, CreateProgram, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createProgram,
    onSuccess: (...args) => {
      queryClient.invalidateQueries(["programs", profileId]);
      options?.onSuccess?.(...args);
    },
  });
  return mutation;
};

export const useUpdateProgram = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<Program, TError, UpdateProgram, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateProgram,
    onSuccess: (...args) => {
      queryClient.invalidateQueries(["programs"]);
      options?.onSuccess?.(...args);
    },
  });
  return mutation;
};

export const useDeleteProgram = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<void, TError, string | number, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: deleteProgram,
    onSuccess: (...args) => {
      queryClient.invalidateQueries(["programs"]);
      options?.onSuccess?.(...args);
    },
  });
  return mutation;
};
