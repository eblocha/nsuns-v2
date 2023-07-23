import { Max } from "./maxes";
import { Reps } from "./reps";
import { bothJson, json, post } from "./util";

export type UpdateRequest = {
  profileId: string;
  movementIds: number[];
};

export type UpdateResponse = {
  maxes: Max[];
  reps: Reps[];
};

const path = "/api/update";

export const runUpdates = (req: UpdateRequest): Promise<UpdateResponse> =>
  post(path, {
    body: JSON.stringify(req),
    headers: bothJson().headers,
  }).then(json());
