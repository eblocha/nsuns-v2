import { acceptJson, bothJson, get, json, post, put } from "./util";

export type Movement = {
  id: string;
  name: string;
  description: string | null;
};

export type CreateMovement = Omit<Movement, "id">;

const path = "/api/movements";

export const getMovements = async (): Promise<Movement[]> => get(path, { headers: acceptJson().headers }).then(json());

export const createMovement = async (movement: CreateMovement): Promise<Movement> =>
  post(path, {
    body: JSON.stringify(movement),
    headers: bothJson().headers,
  }).then(json());

export const updateMovement = async (movement: Movement): Promise<Movement> =>
  put(path, {
    body: JSON.stringify(movement),
    headers: bothJson().headers,
  }).then(json());
