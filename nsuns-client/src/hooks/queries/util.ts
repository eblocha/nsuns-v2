import { CreateQueryResult } from "@tanstack/solid-query";
import { Accessor } from "solid-js";

export const updateInArray = <I>(
  items: I[] | undefined,
  newItem: I,
  predicate: (value: I, index: number, obj: I[]) => boolean
): I[] | undefined => {
  if (!items) return;

  const index = items.findIndex(predicate);
  if (index === -1) return items;

  const newItems = [...items];
  newItems.splice(index, 1, newItem);
  return newItems;
};

export type MergedQueryState = {
  isLoading: Accessor<boolean>;
  isSuccess: Accessor<boolean>;
  isError: Accessor<boolean>;
  error: Accessor<unknown>;
};

export const combineQueries = (
  ...queries: CreateQueryResult[]
): MergedQueryState => {
  return {
    isLoading: () => !queries.every((q) => !q.isLoading),
    isSuccess: () => queries.every((q) => q.isSuccess),
    isError: () => !queries.every((q) => !q.isError),
    error: () => queries.find((q) => q.error)?.error,
  };
};
