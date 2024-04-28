import { Program } from "./program";
import { bothJson, json, post } from "./util";

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
}

const path = "/api/programs/from-template";

export const createProgramFromTemplate = async (template: ProgramTemplate): Promise<Program> =>
  post(path, {
    body: JSON.stringify(template),
    headers: bothJson().headers,
  }).then(json());

export const TEMPLATES: NewProgramTemplate[] = [
  {
    name: "NSuns 5-Day",
    days: [
      {
        sets: [],
      },
      {
        sets: [
          {
            amount: 65,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 8,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 4,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 4,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 4,
            repsIsMinimum: false,
          },
          {
            amount: 80,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 70,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 7,
            repsIsMinimum: false,
          },
          {
            amount: 65,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 8,
            repsIsMinimum: true,
          },
        ],
      },
      {
        sets: [
          {
            amount: 75,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 95,
            description: "Peak set",
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 1,
            repsIsMinimum: true,
          },
          {
            amount: 90,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 80,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 70,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 65,
            description: null,
            movementIndex: 1,
            percentageOfMaxIndex: 1,
            reps: 5,
            repsIsMinimum: true,
          }
        ],
      },
      {
        sets: [
          {
            amount: 75,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 95,
            description: "Peak set",
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 1,
            repsIsMinimum: true,
          },
          {
            amount: 90,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 80,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 70,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 65,
            description: null,
            movementIndex: 2,
            percentageOfMaxIndex: 2,
            reps: 5,
            repsIsMinimum: true,
          }
        ],
      },
      {
        sets: [
          {
            amount: 75,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 95,
            description: "Peak set",
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 1,
            repsIsMinimum: true,
          },
          {
            amount: 90,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 80,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 70,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 65,
            description: null,
            movementIndex: 3,
            percentageOfMaxIndex: 3,
            reps: 3,
            repsIsMinimum: true,
          }
        ],
      },
      {
        sets: [
          {
            amount: 75,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 6,
            repsIsMinimum: false,
          },
          {
            amount: 95,
            description: "Peak set",
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 1,
            repsIsMinimum: true,
          },
          {
            amount: 90,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 85,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 80,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 3,
            repsIsMinimum: false,
          },
          {
            amount: 75,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 70,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 5,
            repsIsMinimum: false,
          },
          {
            amount: 65,
            description: null,
            movementIndex: 0,
            percentageOfMaxIndex: 0,
            reps: 5,
            repsIsMinimum: true,
          }
        ],
      },
      {
        sets: [],
      },
    ],
    movements: [
      {
        type: "new",
        name: "Bench Press",
        description: null,
      },
      {
        type: "new",
        name: "Squat",
        description: null,
      },
      {
        type: "new",
        name: "Overhead Press",
        description: null,
      },
      {
        type: "new",
        name: "Deadlift",
        description: null,
      },
    ],
  },
];
