import { acceptJson, bothJson, del, json, post, put } from "./util";

export type Day = 0 | 1 | 2 | 3 | 4 | 5 | 6;

export type ProgramSet = {
  id: string;
  programId: string;
  movementId: string;
  day: Day;
  ordering: number;
  reps: number | null;
  repsIsMinimum: boolean;
  description: string | null;
  amount: number;
  percentageOfMax: string | null;
};

export type CreateProgramSet = Omit<ProgramSet, "id" | "ordering">;

export type UpdateProgramSet = Omit<ProgramSet, "ordering">;

const path = "/api/sets";

export const createSet = async (set: CreateProgramSet): Promise<ProgramSet> =>
  post(path, {
    body: JSON.stringify(set),
    headers: bothJson().headers,
  }).then(json());

export const updateSet = async (set: UpdateProgramSet): Promise<ProgramSet> =>
  put(path, {
    body: JSON.stringify(set),
    headers: bothJson().headers,
  }).then(json());

export const deleteSet = async (id: string): Promise<ProgramSet> =>
  del(`${path}/${id}`, {
    headers: acceptJson().headers,
  }).then(json());
