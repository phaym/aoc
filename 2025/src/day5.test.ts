import { describe, expect, test } from 'vitest';
import { day5 } from './day5.js';

const TEST_FILE = 'input/day5/example.txt';
describe('day5', () => {
  test('test file', async () => {
    expect(await day5(TEST_FILE)).toBe(3);
  });
});
