import http from "k6/http";
import { check, sleep } from "k6";

const host = __ENV.TARGET_HOST || "host.docker.internal:8080";

export const options = {
  thresholds: {
    http_req_duration: ["p(99) < 3000"],
  },
  // Ramp the number of virtual users up and down
  stages: [
    { duration: "30s", target: 1000 },
    { duration: "1m", target: 1000 },
    { duration: "30s", target: 5000 },
    { duration: "1m", target: 5000 },
    { duration: "30s", target: 1000 },
    { duration: "1m", target: 1000 },
    { duration: "30s", target: 5000 },
    { duration: "1m", target: 5000 },
    { duration: "30s", target: 1000 },
    { duration: "1m", target: 1000 },
  ],
};

function randomName() {
  return Math.random().toString();
}

let profile;
let program;

const checks = {
  "is created": (response) => response.status === 201,
  "body is json": (response) => response.body.startsWith("{")
};

export default function () {
  if (!profile) {
    // create a profile
    const profiles_res = http.post(
      `http://${host}/api/profiles`,
      JSON.stringify({
        name: randomName(),
      }),
      {
        headers: {
          "content-type": "application/json",
        },
      }
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
      {
        headers: {
          "content-type": "application/json",
        },
      }
    );

    check(program_res, checks);

    program = JSON.parse(program_res.body);

    sleep(0.1);
  }

  // get the summary
  const res = http.get(`http://${host}/api/programs/${program.id}`);

  check(res, {
    "is ok": (response) => response.status === 200,
    "body is json": (response) => response.body.startsWith("{")
  });

  sleep(0.1);
}
