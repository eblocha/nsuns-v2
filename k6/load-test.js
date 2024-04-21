import http from "k6/http";
import { check, sleep } from "k6";

const host = __ENV.TARGET_HOST || "host.docker.internal:8080";

const min = 1000;
const max = 5000;

export const options = {
  thresholds: {
    http_req_duration: ["p(99) < 3000"],
  },
  // Ramp the number of virtual users up and down
  stages: [
    { duration: "30s", target: min },
    { duration: "1m", target: min },
    { duration: "30s", target: max },
    { duration: "1m", target: max },
    { duration: "30s", target: min },
    { duration: "1m", target: min },
    { duration: "30s", target: max },
    { duration: "1m", target: max },
    { duration: "30s", target: min },
    { duration: "1m", target: min },
  ],
};

function randomName() {
  return Math.random().toString();
}

let profile;
let program;

const checks = {
  "is created": (response) => response.status === 201,
  "body is json": (response) => response.body.startsWith("{"),
};

export default function () {
  if (!profile) {
    const sets = 8;

    const headers = {
      "content-type": "application/json",
    };

    // create a profile
    const profiles_res = http.post(
      `http://${host}/api/profiles`,
      JSON.stringify({
        name: randomName(),
      }),
      { headers }
    );

    check(profiles_res, checks);

    profile = JSON.parse(profiles_res.body);

    sleep(0.5);

    // create a program
    const program_res = http.post(
      `http://${host}/api/programs`,
      JSON.stringify({
        name: randomName(),
        owner: profile.id,
      }),
      { headers }
    );

    check(program_res, checks);

    program = JSON.parse(program_res.body);

    // add workouts to program
    for (let day = 1; day < 6; day++) {
      sleep(0.5);

      // create a movement for the day
      const res = http.post(
        `http://${host}/api/movements`,
        JSON.stringify({
          name: randomName(),
        }),
        { headers }
      );

      check(res, checks);

      const movement_id = JSON.parse(res.body).id;

      for (let i = 0; i < sets; i++) {
        sleep(0.5);

        // create a set for the movement for the day
        const set_res = http.post(
          `http://${host}/api/sets`,
          JSON.stringify({
            program_id: program.id,
            movement_id,
            day,
            reps: 8,
            amount: 10,
            percentage_of_max: movement_id,
          }),
          { headers }
        );

        check(set_res, checks);
      }
    }

    sleep(1);
  }

  // get the summary
  const res = http.get(`http://${host}/api/programs/${program.id}`);

  check(res, {
    "is ok": (response) => response.status === 200,
    "body is json": (response) => response.body.startsWith("{"),
  });

  sleep(1);
}
