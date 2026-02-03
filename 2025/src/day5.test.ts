import { describe, expect, test } from 'vitest';
import { countRanges, day5 } from './day5.js';

const TEST_FILE = 'input/day5/example.txt';
describe('day5', () => {
  test('test file', async () => {
    expect(await day5(TEST_FILE)).toBe(3);
  });

  test('part 2', () => {
    const ranges = [
      [
        [10, 15],
        [10, 15],
      ],
      [
        [10, 13],
        [10, 15],
      ],
      [
        [10, 15],
        [10, 13],
      ],
      [
        [10, 15],
        [12, 15],
      ],
      [
        [12, 15],
        [10, 15],
      ],
      [
        [10, 15],
        [12, 13],
      ],
      [
        [12, 13],
        [10, 15],
      ],
      [
        [10, 13],
        [12, 15],
      ],
      [
        [12, 15],
        [10, 13],
      ],
      [
        [12, 15],
        [10, 13],
        [11, 12],
      ],
    ];
    for (const range of ranges) {
      expect(countRanges(range)).toBe(6);
    }
  });
});
