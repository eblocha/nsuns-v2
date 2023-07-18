import axios from "axios";

export type Program = {
  id: number;
  name: string | null;
  description: string | null;
  createdOn: number;
};

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
