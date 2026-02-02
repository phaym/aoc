import { describe, test } from 'vitest';
import { day2 } from './day2.js';

export const FILE_PATH = 'input/day2/example.txt';
describe('day2', () => {
  test('test1', async () => {
    await day2(FILE_PATH);
  });
});
