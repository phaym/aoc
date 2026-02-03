import fs from 'node:fs';
import readline from 'node:readline';

const FILE_PATH = 'input/day4/input.txt';
export async function day4(filePath = FILE_PATH, part = 2) {
  const rs = fs.createReadStream(filePath);
  const rl = readline.createInterface(rs);
  const matrix = [];
  for await (const line of rl) {
    console.log(line);
    matrix.push(line.split(''));
  }
  const count = part === 1 ? countMoveable(matrix) : countMoveable2(matrix);
  console.log(`got count: ${count}`);
  return count;
}

function countMoveable2(matrix: string[][]) {
  let moved = true;
  let totalCount = 0;
  while (moved) {
    console.log('moved');
    const count = countMoveable(matrix);
    moved = count > 0;
    totalCount += count;
  }
  return totalCount;
}

function countMoveable(matrix: string[][]) {
  let count = 0;
  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix.length; j++) {
      if (matrix[i][j] !== '@') continue;
      const moveable = isMoveable(matrix, i, j);
      if (moveable) {
        count++;
        matrix[i][j] = 'X';
      }
    }
  }
  return count;
}

function isMoveable(matrix: string[][], x: number, y: number) {
  const startRow = x === 0 ? 0 : x - 1;
  const endRow = x === matrix.length - 1 ? matrix.length - 1 : x + 1;
  const startCol = y === 0 ? 0 : y - 1;
  const endCol = y === matrix.length - 1 ? matrix.length - 1 : y + 1;
  let rollCount = 0;
  for (let i = startRow; i <= endRow; i++) {
    for (let j = startCol; j <= endCol; j++) {
      if (i === x && j === y) continue;
      if (matrix[i][j] === '@') {
        rollCount++;
      }
    }
  }
  return rollCount < 4;
}
