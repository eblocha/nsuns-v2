import { baseHeaders, get, json, post, put } from "./util";

export type Reps = {
  id: number;
  profileId: string;
  movementId: number;
  amount: number;
};

export type CreateReps = Omit<Reps, "id">;

export type UpdateReps = {
  id: number;
  amount: number;
};

const path = "/api/reps";

export const getReps = async (profileId: string): Promise<Reps[]> =>
  get(`${path}?profileId=${encodeURIComponent(profileId)}`).then(json());

export const createReps = async (reps: CreateReps): Promise<Reps> =>
  post(path, {
    body: JSON.stringify(reps),
    headers: baseHeaders,
  }).then(json());

export const updateReps = async (reps: UpdateReps): Promise<Reps> =>
  put(path, {
    body: JSON.stringify(reps),
    headers: baseHeaders,
  }).then(json());
