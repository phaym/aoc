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
    const invalid = validateProductRange(range, 2);
    allInvalid = [...allInvalid, ...invalid];
  }
  const total = allInvalid.reduce((prev, curr) => {
    return prev + curr;
  }, 0);
  console.log(`got total: ${total}`);
};

const validateProductRange = ([start, end]: number[], part = 1): number[] => {
  const invalidInRange: number[] = [];
  console.log(`start: ${start} end: ${end}`);
  for (let i = start; i <= end; i++) {
    const id = i.toString();
    const invalid = part === 1 ? isRepeated(id) : isRepeated2(id);
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

export const isRepeated2 = (id: string) => {
  for (let repeats = 2; repeats <= id.length; repeats++) {
    if (id.length % repeats !== 0) continue;
    if (isRepeatedForGap(id, repeats)) {
      return true;
    }
  }
  return false;
};

const isRepeatedForGap = (id: string, repeats: number) => {
  const pointers = [];
  const gap = id.length / repeats;
  for (let i = 1; i < repeats; i++) {
    pointers.push(i * gap);
  }
  for (let l = 0; l < gap; l++) {
    for (const pointer of pointers) {
      const pos = pointer + l;
      if (id[l] !== id[pos]) return false;
    }
  }
  return true;
};
