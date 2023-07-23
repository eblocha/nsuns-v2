import {
  acceptJson,
  bothJson,
  del,
  get,
  json,
  noContent,
  post,
  put,
  sendJson,
} from "./util";
import { ProgramSet } from "./sets";

export type Program = {
  id: number;
  name: string;
  createdOn: number;
};

export type CreateProgram = {
  name: string;
  owner: string;
};

export type UpdateProgram = Omit<Program, "createdOn">;

export type ProgramSummary = {
  program: Program;
  sets: ProgramSet[];
};

const path = "/api/programs";

export const getProfilePrograms = async (id: string): Promise<Program[]> =>
  get(`${path}?profileId=${encodeURIComponent(id)}`, {
    headers: acceptJson().headers,
  }).then(json());

export const createProgram = async (program: CreateProgram): Promise<Program> =>
  post(path, {
    body: JSON.stringify(program),
    headers: bothJson().headers,
  }).then(json());

export const getProgramSummary = async (
  programId: number | string
): Promise<ProgramSummary> => get(`${path}/${programId}`).then(json());

export const updateProgram = async (program: UpdateProgram): Promise<Program> =>
  put(path, {
    body: JSON.stringify(program),
    headers: bothJson().headers,
  }).then(json());

export const deleteProgram = async (id: number | string): Promise<void> =>
  del(`${path}/${id}`).then(noContent);
