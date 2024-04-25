import { noContent, post } from "./util";

export type Credentials = {
  username: string;
  password: string;
};

export const login = async (credentials: Credentials): Promise<void> =>
  post("/api/auth/login", {
    headers: { Authorization: "Basic " + btoa(`${credentials.username}:${credentials.password}`) },
  }).then(noContent);

export const loginAnonymous = async (): Promise<void> => post("/api/auth/anonymous").then(noContent);

export const logout = async (): Promise<void> => post("/api/auth/logout").then(noContent);
