export const QueryKeys = {
  maxes: (profileId: string) => ["maxes", profileId],
  movements: () => ["movements"],
  profiles: () => ["profiles"],
  programs: {
    // profile ids are uuids, so no collisions
    list: (profileId: string) => ["programs", profileId],
    summary: (programId: string) => ["programs", programId],
  },
  reps: (profileId: string) => ["reps", profileId],
} as const;
