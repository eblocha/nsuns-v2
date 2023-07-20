import axios from "axios";
import { baseHeaders } from "./util";
import { ProgramSet } from "./sets";

export type Program = {
  id: number;
  name: string | null;
  description: string | null;
  createdOn: number;
};

export type CreateProgram = {
  name?: string | null;
  description?: string | null;
  owner: string;
};

export type ProgramSummary = {
  program: Program;
  sets: ProgramSet[];
};

const path = "/api/programs";

export const getUserPrograms = async (id: string): Promise<Program[]> => {
  return (
    await axios.get(path, {
      params: { userId: id },
    })
  ).data;
};

export const createProgram = async (
  program: CreateProgram
): Promise<Program> => {
  return (
    await axios.post(path, program, {
      headers: baseHeaders,
    })
  ).data;
};

export const getProgramSummary = async (
  programId: number | string
): Promise<ProgramSummary> => {
  return (await axios.get(`${path}/${programId}`)).data;
};
