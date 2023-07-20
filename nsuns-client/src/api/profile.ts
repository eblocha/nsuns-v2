import axios from "axios";
import { baseHeaders } from "./util";

export type Profile = {
  id: string;
  name: string;
};

export type CreateProfile = Omit<Profile, "id">;

const path = "/api/profiles";

export const getProfiles = async (): Promise<Profile[]> => {
  return (await axios.get(path)).data;
};

export const getProfile = async (id: string): Promise<Profile> => {
  return (await axios.get(`${path}/${id}`)).data;
};

export const createProfile = async (profile: CreateProfile): Promise<Profile> => {
  return (await axios.post(path, profile, {
    headers: baseHeaders,
  })).data;
};

export const updateProfile = async (profile: Profile): Promise<Profile> => {
  return (
    await axios.put(path, profile, {
      headers: baseHeaders,
    })
  ).data;
};

export const deleteProfile = async (id: string): Promise<void> => {
  return axios.delete(`${path}/${id}`);
};
