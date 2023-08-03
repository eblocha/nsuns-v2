import { Day } from "../util/days";
import { bothJson, del, json, noContent, post, put } from "./util";

export type ProgramSet = {
  id: string;
  programId: string;
  day: Day;
  movementId: string;
  reps: number | null;
  repsIsMinimum: boolean;
  description: string | null;
  amount: number;
  percentageOfMax: string | null;
};

export type CreateProgramSet = Omit<ProgramSet, "id">;

export type UpdateProgramSet = ProgramSet;

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

export const deleteSet = async (id: string): Promise<void> =>
  del(`${path}/${id}`).then(noContent());
