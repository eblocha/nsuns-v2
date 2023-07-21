export const baseHeaders = {
  "content-type": "application/json",
};

export type FetchParams = Parameters<typeof fetch>;

export const processResponse = async <T>(res: Response): Promise<T> => {
  if (!res.ok) {
    throw new Error(
      `HTTP Error ${res.status} (${res.statusText}): ${await res.text()}`
    );
  }

  return await res.json();
};

type Fetcher = <T>(...args: FetchParams) => Promise<T>;

const req =
  (method: string = "GET"): Fetcher =>
  <T>(...args: FetchParams) =>
    fetch(args[0], {
      method: method,
      ...args[1],
    }).then(processResponse) as Promise<T>;

export const get = req();

export const post = req("POST");

export const put = req("PUT");

export const del = req("DELETE");
