import { acceptJson, bothJson, del, get, json, post, put } from "./util";

export type Profile = {
  id: string;
  name: string;
};

export type CreateProfile = Omit<Profile, "id">;

const path = "/api/profiles";

export const getProfiles = async (): Promise<Profile[]> =>
  get(path, { headers: acceptJson().headers }).then(json());

export const getProfile = async (id: string): Promise<Profile> =>
  get(`${path}/${id}`, { headers: acceptJson().headers }).then(json());

export const createProfile = async (profile: CreateProfile): Promise<Profile> =>
  post(path, {
    body: JSON.stringify(profile),
    headers: bothJson().headers,
  }).then(json());

export const updateProfile = async (profile: Profile): Promise<Profile> =>
  put(path, {
    body: JSON.stringify(profile),
    headers: bothJson().headers,
  }).then(json());

export const deleteProfile = async (id: string): Promise<Profile> =>
  del(`${path}/${id}`, {
    headers: acceptJson().headers,
  }).then(json());
