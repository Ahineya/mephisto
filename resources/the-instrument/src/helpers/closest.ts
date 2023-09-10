export const closest = (arr: number[], goal: number) => arr.reduce((prev, curr) => {
  return (Math.abs(curr - goal) < Math.abs(prev - goal) ? curr : prev);
});

export const closestIndex = (arr: number[], n: number) => arr.findIndex(i => i === closest(arr, n));
