export type Program = {
  id: number;
  name: string | null;
  description: string | null;
  createdOn: number;
};

export type UserPrograms = {
  default: Program | null;
  all: Program[];
};

export const getUserPrograms = async (id: string): Promise<UserPrograms> => {
  return (await fetch(`/api/programs?userId=${encodeURIComponent(id)}`)).json();
};
