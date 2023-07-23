import { acceptJson, bothJson, get, json, post, put } from "./util";

export type Reps = {
  id: string;
  profileId: string;
  movementId: string;
  amount: number | null;
};

export type CreateReps = Omit<Reps, "id">;

export type UpdateReps = {
  id: string;
  amount: number | null;
};

const path = "/api/reps";

export const getReps = async (profileId: string): Promise<Reps[]> =>
  get(`${path}?profileId=${encodeURIComponent(profileId)}`, {
    headers: acceptJson().headers,
  }).then(json());

export const createReps = async (reps: CreateReps): Promise<Reps> =>
  post(path, {
    body: JSON.stringify(reps),
    headers: bothJson().headers,
  }).then(json());

export const updateReps = async (reps: UpdateReps): Promise<Reps> =>
  put(path, {
    body: JSON.stringify(reps),
    headers: bothJson().headers,
  }).then(json());
