import { baseHeaders, del, json, noContent, post, put } from "./util";

export type Day = 0 | 1 | 2 | 3 | 4 | 5 | 6;

export type ProgramSet = {
  id: number;
  programId: number;
  movementId: number;
  day: Day;
  ordering: number;
  reps: number | null;
  repsIsMinimum: boolean;
  description: string | null;
  amount: number;
  percentageOfMax: number | null;
};

export type CreateProgramSet = Omit<ProgramSet, "id" | "ordering">;

export type UpdateProgramSet = Omit<ProgramSet, "ordering">;

const path = "/api/sets";

export const createSet = async (set: CreateProgramSet): Promise<ProgramSet> =>
  post(path, {
    body: JSON.stringify(set),
    headers: baseHeaders,
  }).then(json());

export const updateSet = async (set: UpdateProgramSet): Promise<ProgramSet> =>
  put(path, {
    body: JSON.stringify(set),
    headers: baseHeaders,
  }).then(json());

export const deleteSet = async (id: number | string): Promise<void> =>
  del(`${path}/${id}`).then(noContent);
