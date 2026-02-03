import fs from 'node:fs';
import readline from 'node:readline';
export const FILE_PATH = 'input/day3/input.txt';

export const day3 = async (filePath = FILE_PATH) => {
  const rs = fs.createReadStream(filePath);
  const rl = readline.createInterface(rs);
  let totalJoltage = 0;
  for await (const line of rl) {
    const batteries = line.split('').map((val) => +val);
    const joltage = calcJoltage(batteries);
    totalJoltage += joltage;
  }
  console.log('total is ' + totalJoltage);
  return totalJoltage;
};

export function calcJoltage(batteries: number[]) {
  let left = 0;
  let leftIdx = 0;
  for (let l = 0; l < batteries.length - 1; l++) {
    if (batteries[l] > left) {
      left = batteries[l];
      leftIdx = l;
    }
  }
  let right = 0;
  let rightIdx = 0;
  for (let r = batteries.length - 1; r > leftIdx; r--) {
    if (batteries[r] > right) {
      right = batteries[r];
      rightIdx = r;
    }
  }
  return +(left.toString() + right.toString());
}
