import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateProgramSet,
  ProgramSet,
  ProgramSummary,
  createSet,
  deleteSet,
  getProgramSummary,
  updateSet,
} from "../../api";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";

export type ProgramSummaryQueryData = QueryData<
  ReturnType<typeof useProgramSummaryQuery>
>;

export const useProgramSummaryQuery = (programId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.programs.summary(programId()),
    queryFn: () => getProgramSummary(programId()),
    enabled: !!programId(),
  });
};

export const useCreateSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<ProgramSet, TError, CreateProgramSet, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createSet,
    onSuccess: (set, ...args) => {
      options?.onSuccess?.(set, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(set.programId.toString()),
        (summary?: ProgramSummaryQueryData) =>
          summary && {
            ...summary,
            sets: [...summary.sets, set],
          }
      );
    },
  });

  return mutation;
};

export const useEditSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<ProgramSet, TError, CreateProgramSet, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateSet,
    onSuccess: (set, ...args) => {
      options?.onSuccess?.(set, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(set.programId),
        (summary?: ProgramSummaryQueryData) =>
          summary && {
            ...summary,
            sets:
              updateInArray(summary.sets, set, (s) => s.id === set.id) ??
              summary.sets,
          }
      );
    },
  });

  return mutation;
};

export const useDeleteSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<void, TError, string | number, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: deleteSet,
    onSuccess: (v, id, ...args) => {
      options?.onSuccess?.(v, id, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(id),
        (summary?: ProgramSummaryQueryData) =>
          summary && {
            ...summary,
            sets: summary.sets.filter((s) => s.id !== id),
          }
      );
    },
  });
  return mutation;
};
