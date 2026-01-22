import { describe, expect, test } from 'vitest';
import { day1, getNextDialPosition } from './day1.js';

describe('day1 - dial positions', () => {
  type PositionTest = {
    cur: number;
    direction: string;
    amount: number;
    expected: number;
  };
  test('it gets correct next position', () => {
    const tests: PositionTest[] = [
      {
        cur: 50,
        direction: 'L',
        amount: 20,
        expected: 30,
      },
      {
        cur: 50,
        direction: 'L',
        amount: 68,
        expected: 82,
      },
      {
        cur: 50,
        direction: 'R',
        amount: 68,
        expected: 18,
      },
      {
        cur: 50,
        direction: 'R',
        amount: 368,
        expected: 18,
      },
      {
        cur: 50,
        direction: 'L',
        amount: 368,
        expected: 82,
      },
    ];
    for (const test of tests) {
      expect(getNextDialPosition(test.cur, test.direction, test.amount)).toEqual(test.expected);
    }
  });
  test('tests with file', async () => {
    const result = await day1('input/day1/example.txt');
    expect(result).toEqual(3);
  });
});
