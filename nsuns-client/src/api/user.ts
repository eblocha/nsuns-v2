import axios from "axios";
import { baseHeaders } from "./util";

export type User = {
  id: string;
  username: string;
  name: string | null;
  defaultProgram: string | null;
};

export type CreateUser = Omit<User, "id" | "defaultProgram">;

const path = "/api/users";

export const getUsers = async (): Promise<User[]> => {
  return (await axios.get(path)).data;
};

export const getUser = async (id: string): Promise<User> => {
  return (await axios.get(`${path}/${id}`)).data;
};

export const createUser = async (user: CreateUser): Promise<User> => {
  return (await axios.post(path, user, {
    headers: baseHeaders,
  })).data;
};

export const isUsernameTaken = async (username: string): Promise<boolean> => {
  return (
    await axios.get<{ taken: boolean }>(
      `${path}/validation/is-taken/${username}`
    )
  ).data.taken;
};

export const updateUser = async (user: User): Promise<User> => {
  return (
    await axios.put(path, user, {
      headers: baseHeaders,
    })
  ).data;
};

export const deleteUser = async (id: string): Promise<void> => {
  return axios.delete(`${path}/${id}`);
};
