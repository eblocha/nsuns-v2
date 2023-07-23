import { acceptJson, bothJson, del, get, json, post, put } from "./util";
import { ProgramSet } from "./sets";

export type Program = {
  id: string;
  name: string;
  description: string | null;
  owner: string;
};

export type CreateProgram = {
  name: string;
  description: string | null;
  owner: string;
};

export type UpdateProgram = Omit<Program, "owner">;

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
  programId: string
): Promise<ProgramSummary> => get(`${path}/${programId}`).then(json());

export const updateProgram = async (program: UpdateProgram): Promise<Program> =>
  put(path, {
    body: JSON.stringify(program),
    headers: bothJson().headers,
  }).then(json());

export const deleteProgram = async (id: string): Promise<Program> =>
  del(`${path}/${id}`, {
    headers: acceptJson().headers,
  }).then(json());
