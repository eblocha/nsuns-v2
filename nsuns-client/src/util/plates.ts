export type PlateCount = {
  weight: number;
  count: number;
};

const divmod = (x: number, y: number) => [Math.floor(x / y), x % y] as const;

export const calculatePlates = (weight: number, barWeight: number, plates: number[]): PlateCount[] => {
  // Reverse order to prefer larger plates
  const sortedPlates = [...plates].sort((a, b) => (a < b ? 1 : a > b ? -1 : 0));
  const platesToAddToBar: PlateCount[] = [];

  let weightToAdd = weight - barWeight;

  for (const plate of sortedPlates) {
    const [count, remainder] = divmod(weightToAdd, plate * 2); // plates must be added in pairs
    if (count !== 0) {
      platesToAddToBar.push({
        count,
        weight: plate,
      });
    }

    weightToAdd = remainder;
  }

  return platesToAddToBar;
};
