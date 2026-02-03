import { describe, expect, test } from 'vitest';
import { day4 } from './day4.js';

const TEST_FILE = 'input/day4/example.txt';
describe('day4', () => {
  test('test file', async () => {
    expect(await day4(TEST_FILE)).toBe(13);
  });
});
