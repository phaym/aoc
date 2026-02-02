import fs from 'node:fs';
export const FILE_PATH = 'input/day2/input.txt';

export const day2 = async (filePath = FILE_PATH) => {
  const input = fs.readFileSync(filePath, { encoding: 'utf8' });
  let allInvalid: number[] = [];
  for (const values of input.split(',')) {
    const range = values
      .trim()
      .split('-')
      .map((val) => +val);
    const invalid = validateProductRange(range);
    allInvalid = [...allInvalid, ...invalid];
  }
  const total = allInvalid.reduce((prev, curr) => {
    return prev + curr;
  }, 0);
  console.log(`got total: ${total}`);
};

const validateProductRange = ([start, end]: number[]): number[] => {
  const invalidInRange: number[] = [];
  console.log(`start: ${start} end: ${end}`);
  for (let i = start; i <= end; i++) {
    const id = i.toString();
    const invalid = isRepeated(id);
    if (invalid) {
      console.log(`found invalid num: ${id}`);
      invalidInRange.push(+id);
    }
  }
  return invalidInRange;
};

const isRepeated = (id: string) => {
  if (id.length % 2 !== 0) return false;

  for (let l = 0; l < id.length / 2; l++) {
    const r = l + id.length / 2;
    if (id[l] !== id[r]) return false;
  }
  return true;
};
