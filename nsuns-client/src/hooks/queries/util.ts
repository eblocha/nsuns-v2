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
