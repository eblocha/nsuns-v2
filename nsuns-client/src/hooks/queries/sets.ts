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
import { updateInArray } from "./util";
import { QueryKeys } from "./keys";

export const useProgramSummaryQuery = (programId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.programs.summary(programId()),
    queryFn: () => getProgramSummary(programId()),
    enabled: !!programId(),
  });
};

export const useCreateSet = (
  options?: Partial<
    CreateMutationOptions<ProgramSet, unknown, CreateProgramSet, unknown>
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
        (summary?: ProgramSummary) =>
          summary && {
            ...summary,
            sets: [...summary.sets, set],
          }
      );
    },
  });

  return mutation;
};

export const useEditSet = (
  options?: Partial<
    CreateMutationOptions<ProgramSet, unknown, CreateProgramSet, unknown>
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
        (summary?: ProgramSummary) =>
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

export const useDeleteSet = (
  programId: string,
  options?: Partial<
    CreateMutationOptions<void, unknown, string | number, unknown>
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
        (summary?: ProgramSummary) =>
          summary && {
            ...summary,
            sets: summary.sets.filter((s) => s.id !== id),
          }
      );
    },
  });
  return mutation;
};
