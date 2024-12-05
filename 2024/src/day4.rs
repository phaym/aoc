pub mod part1 {
    use std::{fs, str::Chars};

    pub fn run(file_path: &str) {
        println!("running day 4 part 1");
        let input = fs::read_to_string(file_path).unwrap();
        let mut total = count_horizontal(&input);
        total += count_vertical(&input);

        println!("total :{}", total);
    }

    pub fn count_vertical(puzzle: &str) -> i32 {
        let lines: Vec<&str> = puzzle.lines().collect();

        let mut count = 0;
        for col in 0..lines.len() {
            let mut vertical_word = String::new();
            for line in &lines {
                vertical_word.push(line.chars().nth(col).unwrap());
            }
            count += vertical_word.match_indices("XMAS").count() as i32
                + vertical_word.matches("SAMX").count() as i32;
        }
        count
    }

    pub fn count_horizontal(puzzle: &str) -> i32 {
        puzzle
            .lines()
            .into_iter()
            .map(|line| {
                let mut count = line.matches("XMAS").count() as i32;
                count += line.matches("SAMX").count() as i32;
                count
            })
            .sum()
    }
}

pub mod part2 {
    pub fn run(_file_path: &str) {
        println!("running day 4 part 2");
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    pub fn count_horizontal() {
        let expected = 5;
        let result = part1::count_horizontal(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }
    #[test]
    pub fn count_vertical() {
        let expected = 3;
        let result = part1::count_vertical(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }
}
