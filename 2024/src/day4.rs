pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) {
        println!("running day 4 part 1");
        let input = fs::read_to_string(file_path).unwrap();
        let total = get_counts(&input);
        println!("total :{}", total);
    }

    pub fn get_counts(puzzle: &str) -> i32 {
        let horizontal: i32 = puzzle.lines().map(|line| count_matches(line)).sum();
        horizontal
            + count_vertical(puzzle)
            + count_diagonal(puzzle, true)
            + count_diagonal(puzzle, false)
    }

    pub fn count_matches(line: &str) -> i32 {
        line.matches("XMAS").count() as i32 + line.matches("SAMX").count() as i32
    }

    pub fn count_vertical(puzzle: &str) -> i32 {
        puzzle
            .lines()
            .into_iter()
            .fold(
                vec![String::from(""); puzzle.lines().count()],
                |mut vertical_lines, line| {
                    line.chars()
                        .enumerate()
                        .for_each(|(i, char)| vertical_lines[i].push(char));
                    vertical_lines
                },
            )
            .iter()
            .map(|vert| count_matches(vert.as_str()))
            .sum()
    }

    pub fn count_diagonal(puzzle: &str, reverse: bool) -> i32 {
        let matrix: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();
        let mut count = 0;

        let mut row = 0;
        let mut col = if reverse { matrix.len() - 1 } else { 0 };
        let col_end = if reverse { 0 } else { matrix.len() - 1 };
        loop {
            let mut diagonal_word = String::new();
            let (mut char_col, mut char_row) = (col, row);
            loop {
                diagonal_word.push(matrix[char_row][char_col]);
                if char_col == col_end || char_row == 0 {
                    break;
                }
                char_row -= 1;
                char_col = if reverse { char_col - 1 } else { char_col + 1 };
            }
            count += count_matches(&diagonal_word);
            if row == matrix.len() - 1 && col == col_end {
                break;
            }
            if row < matrix.len() - 1 {
                row += 1;
            } else {
                col = if reverse { col - 1 } else { col + 1 };
            }
        }
        count
    }
}

pub mod part2 {
    use std::fs;

    pub fn run(file_path: &str) -> i32 {
        println!("running day 4 part 2");
        let input = fs::read_to_string(file_path).unwrap();
        let count = count_x_mas(&input);
        println!("count: {}", count);
        count
    }

    pub fn count_x_mas(puzzle: &str) -> i32 {
        let matrix: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();
        let mut count = 0;
        for i in 1..matrix.len() - 1 {
            for j in 1..matrix.len() - 1 {
                if matrix[i][j] == 'A' {
                    let mut cross_one = vec![matrix[i + 1][j - 1], matrix[i - 1][j + 1]];
                    let mut cross_two = vec![matrix[i + 1][j + 1], matrix[i - 1][j - 1]];
                    cross_one.sort();
                    cross_two.sort();
                    if cross_one == cross_two && cross_two == vec!['M', 'S'] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

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
        let result: i32 = INPUT.lines().map(|line| part1::count_matches(line)).sum();
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }
    #[test]
    pub fn count_vertical() {
        let expected = 3;
        let result = part1::count_vertical(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }

    #[test]
    pub fn count_diagonal_up_left() {
        let expected = 5;
        let result = part1::count_diagonal(INPUT, false);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }

    #[test]
    pub fn count_diagonal_up_right() {
        let expected = 5;
        let result = part1::count_diagonal(INPUT, true);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }

    #[test]
    pub fn run_test_file_1() {
        let expected = 18;
        let result = part1::get_counts(INPUT);
        assert_eq!(result, expected, "got{}, expected:{}", result, expected);
    }
    #[test]
    pub fn test_file() {
        let input = "\
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
        let expected = 9;
        let result = part2::count_x_mas(input);
        assert_eq!(result, expected, "got:{}, expected: {}", result, expected);
    }

    #[test]
    pub fn test_run() {
        let expected = 1875;
        let result = part2::run("./day4.txt");
        assert_eq!(expected, result);
    }
}
