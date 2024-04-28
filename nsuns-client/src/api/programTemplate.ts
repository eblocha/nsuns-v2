import { Program } from "./program";
import { bothJson, json, post } from "./util";
import nsuns5day from "./templates/nsuns-5day";

export type SetTemplate = {
  movementIndex: number;
  percentageOfMaxIndex: number | null;
  reps: number | null;
  repsIsMinimum: boolean;
  description: string | null;
  amount: number;
};

export type DayTemplate = {
  sets: SetTemplate[];
};

export type MovementRef = {
  type: "existing";
  id: string;
};

export type NewMovement = {
  type: "new";
  name: string;
  description: string | null;
};

export type MovementTemplate = MovementRef | NewMovement;

export type ProgramTemplate = {
  name: string;
  owner: string;
  // Sunday -> Saturday
  days: [DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate];
  movements: MovementTemplate[];
};

export type NewProgramTemplate = {
  name: string;
  // Sunday -> Saturday
  days: [DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate, DayTemplate];
  movements: NewMovement[];
};

const path = "/api/programs/from-template";

export const createProgramFromTemplate = async (template: ProgramTemplate): Promise<Program> =>
  post(path, {
    body: JSON.stringify(template),
    headers: bothJson().headers,
  }).then(json());

export const TEMPLATES: NewProgramTemplate[] = [nsuns5day];
