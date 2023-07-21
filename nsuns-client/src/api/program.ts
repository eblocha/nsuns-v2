import axios from "axios";
import { baseHeaders } from "./util";
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

export type ProgramSummary = {
  program: Program;
  sets: ProgramSet[];
};

const path = "/api/programs";

export const getProfilePrograms = async (id: string): Promise<Program[]> => {
  return (
    await axios.get(path, {
      params: { profileId: id },
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

export const updateProgram = async (
  program: Omit<Program, "createdOn">
): Promise<Program> => {
  return (await axios.put(path, program)).data;
};

export const deleteProgram = async (id: number | string): Promise<void> => {
  return await axios.delete(`${path}/${id}`);
};
