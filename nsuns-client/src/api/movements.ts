import axios from "axios";

export type Movement = {
  id: number;
  name: string;
  description: string | null;
};

export type CreateMovement = Omit<Movement, "id">;

const path = "/api/movements";

export const getMovements = async (): Promise<Movement[]> => {
  return (await axios.get(path)).data;
};

export const createMovement = async (
  movement: CreateMovement
): Promise<Movement> => {
  return (await axios.post(path, movement)).data;
};
