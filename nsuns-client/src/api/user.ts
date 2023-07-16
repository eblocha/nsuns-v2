export type User = {
  id: string;
  username: string;
  name: string | null;
  defaultProgram: string | null;
};

export type CreateUser = Omit<User, "id" | "defaultProgram">;

const path = "/api/users";

export const getUsers = async (): Promise<User[]> => {
  return (await fetch(path)).json();
};

export const createUser = async (user: CreateUser): Promise<User> => {
  return (
    await fetch(path, {
      method: "POST",
      body: JSON.stringify(user),
      headers: {
        "content-type": "application/json",
      },
    })
  ).json();
};

export const isUsernameTaken = async (username: string): Promise<boolean> => {
  return (await (await fetch(`${path}/validation/is-taken/${username}`)).json())
    .taken;
};

export const updateUser = async (user: User): Promise<User> => {
  return (
    await fetch(path, {
      method: "PUT",
      body: JSON.stringify(user),
      headers: {
        "content-type": "application/json",
      },
    })
  ).json();
};

export const deleteUser = async (id: string): Promise<void> => {
  await fetch(`${path}/${id}`, { method: "DELETE" });
};
