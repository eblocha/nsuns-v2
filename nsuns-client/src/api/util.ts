export const baseHeaders = {
  "content-type": "application/json",
};

export type FetchParams = Parameters<typeof fetch>;

export const processResponse = async (res: Response): Promise<Response> => {
  if (!res.ok) {
    throw new Error(
      `HTTP Error ${res.status} (${res.statusText}): ${await res.text()}`
    );
  }

  return res;
};

export const noContent = () => undefined;

export const json = <T>() => (res: Response): Promise<T> => res.json() as Promise<T>;

type Fetcher = (...args: FetchParams) => Promise<Response>;

const req =
  (method: string = "GET"): Fetcher =>
  (...args: FetchParams) =>
    fetch(args[0], {
      method: method,
      ...args[1],
    }).then(processResponse);

export const get = req();

export const post = req("POST");

export const put = req("PUT");

export const del = req("DELETE");
