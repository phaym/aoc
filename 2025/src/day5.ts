import fs from 'node:fs';
import readline from 'node:readline';

const FILE_PATH = 'input/day5/input.txt';
export async function day5(filePath = FILE_PATH) {
  const rs = fs.createReadStream(filePath);
  const rl = readline.createInterface(rs);
  const freshRanges = [];
  const ingredients = [];
  for await (const line of rl) {
    if (!line.length) continue;
    const parsedLine = line.split('-').map((val) => +val);
    if (parsedLine.length > 1) {
      freshRanges.push(parsedLine);
    } else {
      ingredients.push(...parsedLine);
    }
  }
  const freshCount = countFreshIngredients(ingredients, freshRanges);
  const part2 = countRanges(freshRanges);
  console.log(`total count ${freshCount} part2:${part2}`);
  return freshCount;
}

export const countRanges = (freshRanges: number[][]) => {
  const sorted = freshRanges.sort((a, b) => a[0] - b[0]);
  let start = sorted[0][0];
  let end = sorted[0][1];
  let total = 0;
  for (let i = 1; i < freshRanges.length; i++) {
    const currStart = sorted[i][0];
    const currEnd = sorted[i][1];
    if (currStart <= end) {
      end = Math.max(currEnd, end);
    } else {
      total += end - start + 1;
      start = currStart;
      end = currEnd;
    }
  }
  total += end - start + 1;
  return total;
};

function countFreshIngredients(ingredients: number[], freshRanges: number[][]) {
  let freshCount = 0;
  for (const ingredient of ingredients) {
    const isFresh = checkIsFresh(ingredient, freshRanges);
    if (isFresh) freshCount++;
  }
  return freshCount;
}

function checkIsFresh(ingredient: number, freshRanges: number[][]) {
  for (const range of freshRanges) {
    const [start, end] = range;
    if (ingredient >= start && ingredient <= end) {
      return true;
    }
  }
  return false;
}
