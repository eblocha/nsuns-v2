import { Max } from "./maxes";
import { Reps } from "./reps";
import { bothJson, del, json, post } from "./util";

export type UpdateRequest = {
  profileId: string;
  movementIds: string[];
};

export type UpdateResponse = {
  maxes: Max[];
  reps: Reps[];
};

export type UndoResponse = {
  maxes: string[];
  reps: string[];
};

const path = "/api/updates";

export const runUpdates = (req: UpdateRequest): Promise<UpdateResponse> =>
  post(path, {
    body: JSON.stringify(req),
    headers: bothJson().headers,
  }).then(json());

export const undoUpdates = (req: UpdateRequest): Promise<UndoResponse> =>
  del(path, {
    body: JSON.stringify(req),
    headers: bothJson().headers,
  }).then(json());
