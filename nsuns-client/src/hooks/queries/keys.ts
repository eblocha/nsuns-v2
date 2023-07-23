export const QueryKeys = {
  maxes: (profileId: string) => ["maxes", profileId],
  movements: () => ["movements"],
  programs: {
    // profile ids are uuids, so no collisions
    list: (profileId: string) => ["programs", profileId],
    summary: (programId: string | number) => ["programs", programId.toString()],
  },
  reps: (profileId: string) => ["reps", profileId]
} as const;
