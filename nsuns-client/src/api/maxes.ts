import { acceptJson, bothJson, get, json, post, put } from "./util";

export type Max = {
  id: number;
  profileId: string;
  movementId: number;
  amount: number;
};

export type CreateMax = Omit<Max, "id">;

export type UpdateMax = {
  id: number;
  amount: number;
};

const path = "/api/maxes";

export const getMaxes = async (profileId: string): Promise<Max[]> =>
  get(`${path}?profileId=${encodeURIComponent(profileId)}`, {
    headers: acceptJson().headers,
  }).then(json());

export const createMax = async (max: CreateMax): Promise<Max> =>
  post(path, {
    body: JSON.stringify(max),
    headers: bothJson().headers,
  }).then(json());

export const updateMax = async (max: UpdateMax): Promise<Max> =>
  put(path, {
    body: JSON.stringify(max),
    headers: bothJson().headers,
  }).then(json());
