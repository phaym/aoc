import { describe, expect, test } from 'vitest';
import { calcJoltage, day3 } from './day3.js';

const TEST_FILE = 'input/day3/example.txt';
describe('day3', () => {
  test('test file', async () => {
    expect(await day3(TEST_FILE)).toBe(357);
  });

  test('joltage', () => {
    expect(calcJoltage([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1])).toBe(98);
    expect(calcJoltage([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9])).toBe(89);
    expect(calcJoltage([1, 1, 1, 6, 8, 1, 1, 1, 3, 1, 1, 1, 1, 1])).toBe(83);
    expect(calcJoltage([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8])).toBe(78);
  });
});
