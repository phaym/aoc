pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) {
        println!("running day 4 part 1");
        let input = fs::read_to_string(file_path).unwrap();

        let total = get_counts(&input);
        println!("total :{}", total);
    }

    pub fn get_counts(puzzle: &str) -> i32 {
        count_horizontal(&puzzle)
            + count_vertical(&puzzle)
            + count_diagonal_left(&puzzle)
            + count_diagonal_right(&puzzle)
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

    pub fn count_diagonal_left(puzzle: &str) -> i32 {
        let lines: Vec<&str> = puzzle.lines().collect();

        let mut count = 0;
        let mut row = 0;
        let mut col = lines.len() - 1;
        while row < lines.len() - 1 || col > 0 {
            let mut diagonal_word = String::new();
            let mut curr_col = col;
            let mut curr_row = row;
            loop {
                println!("left coords row:{}, col:{}", curr_row, curr_col);
                diagonal_word.push(lines[curr_row].chars().nth(curr_col).unwrap());
                if curr_row == 0 || curr_col == 0 {
                    break;
                } else {
                    curr_row -= 1;
                    curr_col -= 1;
                }
            }
            println!("next");
            count += diagonal_word.match_indices("XMAS").count() as i32
                + diagonal_word.matches("SAMX").count() as i32;
            if row < lines.len() - 1 {
                row += 1;
            } else {
                col -= 1;
            }
        }
        count
    }

    pub fn count_diagonal_right(puzzle: &str) -> i32 {
        let lines: Vec<&str> = puzzle.lines().collect();

        let mut count = 0;
        let mut row = 0;
        let mut col = 0;
        while row < lines.len() - 1 || col < lines.len() {
            let mut diagonal_word = String::new();
            let mut curr_col = col;
            let mut curr_row = row;
            loop {
                println!("right coords row:{}, col:{}", curr_row, curr_col);
                diagonal_word.push(lines[curr_row].chars().nth(curr_col).unwrap());
                if curr_row == 0 || curr_col == lines.len() - 1 {
                    break;
                } else {
                    curr_row -= 1;
                    curr_col += 1;
                }
            }
            println!("next");
            count += diagonal_word.match_indices("XMAS").count() as i32
                + diagonal_word.matches("SAMX").count() as i32;
            if row < lines.len() - 1 {
                row += 1;
            } else {
                col += 1;
            }
        }
        count
    }

    pub fn count_horizontal(puzzle: &str) -> i32 {
        puzzle
            .lines()
            .into_iter()
            .map(|line| {
                let count =
                    line.matches("XMAS").count() as i32 + line.matches("SAMX").count() as i32;
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

    #[test]
    pub fn count_diagonal_left() {
        let expected = 5;
        let result = part1::count_diagonal_left(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }

    #[test]
    pub fn count_diagonal_right() {
        let expected = 5;
        let result = part1::count_diagonal_right(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }

    #[test]
    pub fn run_test_file_1() {
        let expected = 18;
        let result = part1::get_counts(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }
}
