import { baseHeaders, del, get, json, noContent, post, put } from "./util";

export type Profile = {
  id: string;
  name: string;
};

export type CreateProfile = Omit<Profile, "id">;

const path = "/api/profiles";

export const getProfiles = async (): Promise<Profile[]> =>
  get(path).then(json());

export const getProfile = async (id: string): Promise<Profile> =>
  get(`${path}/${id}`).then(json());

export const createProfile = async (profile: CreateProfile): Promise<Profile> =>
  post(path, {
    body: JSON.stringify(profile),
    headers: baseHeaders,
  }).then(json());

export const updateProfile = async (profile: Profile): Promise<Profile> =>
  put(path, {
    body: JSON.stringify(profile),
    headers: baseHeaders,
  }).then(json());

export const deleteProfile = async (id: string): Promise<void> =>
  del(`${path}/${id}`).then(noContent);
