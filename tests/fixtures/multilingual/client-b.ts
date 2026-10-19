import { Policy } from './policy';

export class ClientB {
  run(): void { Policy.check(); }
}

export function adjustment(input: number): number {
  const scaled = input * 2;
  const total = scaled + 5;
  return total;
}
