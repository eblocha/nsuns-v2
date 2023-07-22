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

export const useProgramSummaryQuery = (programId: Accessor<string>) => {
  return createQuery({
    queryKey: () => ["programs", programId()],
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
        ["programs", set.programId.toString()],
        (summary?: ProgramSummary) => {
          if (!summary) return;

          return {
            program: summary.program,
            sets: [...summary.sets, set],
          };
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
        ["programs", set.programId.toString()],
        (summary?: ProgramSummary) => {
          if (!summary) return;

          return {
            program: summary.program,
            sets:
              updateInArray(summary.sets, set, (s) => s.id === set.id) ??
              summary.sets,
          };
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
        ["programs", programId],
        (summary?: ProgramSummary) => {
          if (!summary) return;
          return {
            program: summary.program,
            sets: summary.sets.filter((s) => s.id !== id),
          };
        }
      );
    },
  });
  return mutation;
};
