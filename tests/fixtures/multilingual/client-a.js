import { Policy } from './policy.js';

export class ClientA {
  run() { Policy.check(); }
}

export function adjustment(value) {
  const doubled = value * 2;
  const result = doubled + 5;
  return result;
}
