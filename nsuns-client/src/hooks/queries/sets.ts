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

export const useProgramSummaryQuery = (programId: string) => {
  return createQuery({
    queryKey: () => ["programs", programId],
    queryFn: () => getProgramSummary(programId),
    enabled: !!programId,
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
      options?.onSuccess?.(set, ...args);
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
      queryClient.setQueryData(
        ["programs", set.programId.toString()],
        (summary?: ProgramSummary) => {
          if (!summary) return;

          const index = summary.sets.findIndex((s) => s.id === set.id);

          if (index === -1) return;

          const newSets = [...summary.sets];
          newSets.splice(index, 1, set);

          return {
            program: summary.program,
            sets: newSets,
          };
        }
      );
      options?.onSuccess?.(set, ...args);
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
      options?.onSuccess?.(v, id, ...args);
    },
  });
  return mutation;
};
