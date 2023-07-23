import { Max } from "./maxes";
import { Reps } from "./reps";
import { bothJson, del, json, noContent, post, sendJson } from "./util";

export type UpdateRequest = {
  profileId: string;
  movementIds: number[];
};

export type UpdateResponse = {
  maxes: Max[];
  reps: Reps[];
};

const path = "/api/updates";

export const runUpdates = (req: UpdateRequest): Promise<UpdateResponse> =>
  post(path, {
    body: JSON.stringify(req),
    headers: bothJson().headers,
  }).then(json());

export const undoUpdates = (req: UpdateRequest): Promise<void> =>
  del(path, {
    body: JSON.stringify(req),
    headers: sendJson().headers,
  }).then(noContent());
