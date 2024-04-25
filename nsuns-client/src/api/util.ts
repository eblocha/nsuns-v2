import { ApiError } from "./error";

export const applicationJson = "application/json";

export class HeaderBuilder {
  readonly headers: Record<string, string> = {};

  contentType(type: string): HeaderBuilder {
    this.headers["Content-Type"] = type;
    return this;
  }

  accept(type: string): HeaderBuilder {
    this.headers["Accept"] = type;
    return this;
  }

  header(name: string, value: string): HeaderBuilder {
    this.headers[name] = value;
    return this;
  }
}

export const bothJson = () => new HeaderBuilder().contentType(applicationJson).accept(applicationJson);

export const acceptJson = () => new HeaderBuilder().accept(applicationJson);

export const sendJson = () => new HeaderBuilder().contentType(applicationJson);

export type FetchParams = Parameters<typeof fetch>;

const getErrorMessage = async (res: Response): Promise<string> => {
  const contentType = res.headers.get("content-type");

  if (contentType?.startsWith("text/html")) return "";

  return await res.text();
};

export const processResponse = async (res: Response): Promise<Response> => {
  if (res.type === "opaqueredirect") {
    window.location.href = res.url;
  }

  if (!res.ok) {
    const msg = await getErrorMessage(res);
    throw new ApiError(res.status, res.statusText, msg);
  }

  return res;
};

export const noContent = () => undefined;

export const json =
  <T>() =>
  (res: Response): Promise<T> =>
    res.json() as Promise<T>;

type Fetcher = (...args: FetchParams) => Promise<Response>;

const req =
  (method: string = "GET"): Fetcher =>
  (...args: FetchParams) =>
    fetch(args[0], {
      method: method,
      redirect: "manual",
      ...args[1],
    }).then(processResponse);

export const get = req();

export const post = req("POST");

export const put = req("PUT");

export const del = req("DELETE");
