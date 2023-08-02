import { Day } from "../util/days";
import { bothJson, del, json, noContent, post, put, sendJson } from "./util";

export type ProgramSet = {
  id: string;
  movementId: string;
  reps: number | null;
  repsIsMinimum: boolean;
  description: string | null;
  amount: number;
  percentageOfMax: string | null;
};

export type CreateProgramSet = Omit<ProgramSet, "id"> & {
  programId: string;
  day: Day;
};

export type UpdateProgramSet = ProgramSet;

export type SetDeleteMeta = {
  day: Day;
  programId: string;
};

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

export const deleteSet = async (
  id: string,
  meta: SetDeleteMeta
): Promise<void> =>
  del(`${path}/${id}`, {
    body: JSON.stringify(meta),
    headers: sendJson().headers,
  }).then(noContent());
