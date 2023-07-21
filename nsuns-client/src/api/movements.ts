import { baseHeaders, get, post } from "./util";

export type Movement = {
  id: number;
  name: string;
  description: string | null;
};

export type CreateMovement = Omit<Movement, "id">;

const path = "/api/movements";

export const getMovements = async (): Promise<Movement[]> => get(path);

export const createMovement = async (
  movement: CreateMovement
): Promise<Movement> =>
  post(path, {
    body: JSON.stringify(movement),
    headers: baseHeaders,
  });
