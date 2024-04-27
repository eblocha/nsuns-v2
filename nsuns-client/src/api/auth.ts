import { get, json, noContent, post } from "./util";

export type Credentials = {
  username: string;
  password: string;
};

export type UserInfo = {
  type: "user";
  id: string;
  username: string;
};

export type TemporaryInfo = {
  type: "anonymous";
  expiryDate: number;
};

export const login = async (credentials: Credentials): Promise<void> =>
  post("/api/auth/login", {
    headers: { Authorization: "Basic " + btoa(`${credentials.username}:${credentials.password}`) },
  }).then(noContent);

export const loginAnonymous = async (): Promise<void> => post("/api/auth/anonymous").then(noContent);

export const logout = async (): Promise<void> => post("/api/auth/logout").then(noContent);

export const userInfo = async (): Promise<UserInfo | TemporaryInfo> =>
  get("/api/auth/user-info").then(json<UserInfo | TemporaryInfo>());
