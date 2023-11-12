import http from "k6/http";
import { sleep } from "k6";

const host = __ENV.TARGET_HOST || "host.docker.internal:8080";

export const options = {
  thresholds: {
    http_req_duration: ["p(99) < 3000"],
  },
  // Ramp the number of virtual users up and down
  stages: [
    { duration: "30s", target: 200 },
    { duration: "1m", target: 500 },
    { duration: "1m", target: 500 },
    { duration: "30s", target: 200 },
  ],
};

function randomName() {
  return Math.random().toString();
}

let profile;
let program;

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

    program = JSON.parse(program_res.body);

    sleep(0.1);
  }

  // get the summary
  http.get(`http://${host}/api/programs/${program.id}`);

  // sleep(0.1);
}
