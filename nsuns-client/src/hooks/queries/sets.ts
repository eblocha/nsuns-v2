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
  getProgramSummary,
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
