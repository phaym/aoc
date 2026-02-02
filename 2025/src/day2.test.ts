import { describe, expect, test } from 'vitest';
import { day2, isRepeated2 } from './day2.js';

export const FILE_PATH = 'input/day2/example.txt';

describe('day2', () => {
  test('test1', async () => {
    await day2(FILE_PATH);
  });

  describe('part2', () => {
    test('repeated', () => {
      expect(isRepeated2('123123')).toBe(true);
      expect(isRepeated2('123123123')).toBe(true);
      expect(isRepeated2('1231231234')).toBe(false);
      expect(isRepeated2('123123123123123')).toBe(true);
      expect(isRepeated2('1212121212')).toBe(true);
    });
    test('special', () => {
      expect(isRepeated2('1111111')).toBe(true);
    });
  });
});
