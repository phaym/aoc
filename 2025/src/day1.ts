import fs from 'node:fs';
import readline from 'node:readline';

export const FILE_PATH = 'input/day1/example.txt';

type Rotation = {
  dir: string;
  amount: number;
};
export const day1 = async (filePath = FILE_PATH) => {
  const fileStream = fs.createReadStream(filePath, {
    encoding: 'utf8',
  });
  const rl = readline.createInterface({
    input: fileStream,
  });

  const rotations = await parseFile(rl);
  let cur = 50;
  let zeroCount = 0;
  for (const { dir, amount } of rotations) {
    cur = getNextDialPosition(cur, dir, amount);
    console.log(`next position: ${cur} from ${dir}${amount}`);
    if (cur === 0 || cur === 100) {
      zeroCount++;
    }
  }
  console.log(`final: ${zeroCount}`);
  return zeroCount;
};

export const getNextDialPosition = (currentDial: number, dir: string, amount: number): number => {
  if (dir === 'L') {
    if (currentDial - amount < 0) {
      return 100 - (amount % currentDial);
    } else {
      return currentDial - amount;
    }
  } else {
    if (currentDial + amount > 100) {
      return (currentDial + amount) % 100;
    } else {
      return currentDial + amount;
    }
  }
};

const parseFile = async (rl: readline.Interface): Promise<Rotation[]> => {
  const rotations: Rotation[] = [];
  for await (const line of rl) {
    const [dir, ...rest] = line;
    const rotation: Rotation = {
      dir,
      amount: Number.parseInt(rest.join(''), 10),
    };
    rotations.push(rotation);
  }
  return rotations;
};
