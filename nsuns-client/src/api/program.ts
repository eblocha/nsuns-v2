import axios from "axios";
import { baseHeaders } from "./util";

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
}

export type UserPrograms = {
  default: Program | null;
  all: Program[];
};

const path = "/api/programs";

export const getUserPrograms = async (id: string): Promise<UserPrograms> => {
  return (
    await axios.get(path, {
      params: { userId: id },
    })
  ).data;
};

export const createProgram = async (
  program: CreateProgram
): Promise<Program> => {
  return (await axios.post(path, program, {
    headers: baseHeaders,
  })).data;
};
