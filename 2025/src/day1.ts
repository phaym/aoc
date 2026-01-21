import fs from 'node:fs';

const FILE_PATH = 'input/day1/example.txt';

export const day1 = async () => {
  const input = fs.createReadStream(FILE_PATH, {
    encoding: 'utf8',
  });
  for await (const line of input) {
    console.log(line);
  }
};
