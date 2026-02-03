import fs from 'node:fs';
import readline from 'node:readline';

const FILE_PATH = 'input/day5/input.txt';
export async function day5(filePath = FILE_PATH) {
  const rs = fs.createReadStream(filePath);
  const rl = readline.createInterface(rs);
  const freshRanges = [];
  const ingredients = [];
  let reading_ranges = true;
  for await (const line of rl) {
    if (line === '') {
      reading_ranges = false;
    } else if (reading_ranges) {
      freshRanges.push(line.split('-').map((val) => +val));
    } else {
      ingredients.push(+line);
    }
  }
  const freshCount = countFreshIngredients(ingredients, freshRanges);
  console.log(`total count ${freshCount}`);
  return freshCount;
}

function countFreshIngredients(ingredients: number[], fresh_ranges: number[][]) {
  let freshCount = 0;
  for (const ingredient of ingredients) {
    const isFresh = checkIsFresh(ingredient, fresh_ranges);
    if (isFresh) freshCount++;
  }
  return freshCount;
}

function checkIsFresh(ingredient: number, fresh_ranges: number[][]) {
  for (const range of fresh_ranges) {
    const [start, end] = range;
    if (ingredient >= start && ingredient <= end) {
      console.log(`ingredient: ${ingredient} start: ${start} end: ${end}`);
      return true;
    }
  }
  return false;
}
