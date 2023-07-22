import { baseHeaders, get, json, post } from "./util";

export type Max = {
  id: number;
  profileId: string;
  movementId: number;
  amount: number | null;
};

export type CreateMax = Omit<Max, "id">;

const path = "/api/maxes";

export const getMaxes = async (profileId: string): Promise<Max[]> =>
  get(`${path}?profileId=${encodeURIComponent(profileId)}`).then(json());

export const createMax = async (max: CreateMax): Promise<Max> =>
  post(path, {
    body: JSON.stringify(max),
    headers: baseHeaders,
  }).then(json());
