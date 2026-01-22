import { describe, expect, test } from 'vitest';
import { day1, getNextDialPosition } from './day1.js';

describe('day1 - dial positions', () => {
  test.each([
    [50, 'L', 20, 30],
    [50, 'L', 68, 82],
    [50, 'R', 68, 18],
    [50, 'R', 368, 18],
    [50, 'L', 368, 82],
    [0, 'L', 300, 0],
    [20, 'R', 180, 0],
    [20, 'L', 120, 0],
    [61, 'R', 39, 0],
  ])('it gets correct next position cur cur:%d dir:%s amount:%d exp:%d', (cur, direction, amount, expected) => {
    expect(getNextDialPosition(cur, direction, amount)).toEqual(expected);
  });

  test('tests with file', async () => {
    const result = await day1('input/day1/example.txt');
    expect(result).toEqual(3);
  });
});
