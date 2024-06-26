export type JsonError = {
  path: string;
  message: string;
  status: number;
};

export class ApiError {
  constructor(
    readonly status: number,
    readonly statusText: string,
    readonly cause: string
  ) {}

  toString() {
    return `HTTP Status ${this.status} (${this.statusText}) ${this.cause}`;
  }

  get [Symbol.toStringTag]() {
    return "ApiError";
  }
}

export class RedirectError {
  constructor(readonly url: string) {}
}

export const isNotFound = (error: unknown): boolean => error instanceof ApiError && error.status === 404;
