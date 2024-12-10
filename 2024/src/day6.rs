pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> i32 {
        let map = fs::read_to_string(file_path).unwrap();
        let (matrix, start_pos) = parse_matrix(map);
        let count = count_moves(matrix, start_pos);
        println!("count is: {}", count);
        count
    }

    pub fn count_moves(mut matrix: Vec<Vec<char>>, (mut row, mut col): (isize, isize)) -> i32 {
        let max = matrix.len() as isize;

        let mut count = 0;
        let deltas: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut delta_idx = 0;
        while row < max && row >= 0 && col < max && col >= 0 {
            if matrix[row as usize][col as usize] != 'X' {
                count += 1;
            }
            matrix[row as usize][col as usize] = 'X';

            let mut delta = deltas[delta_idx];
            if let Some('#') = try_get_char(&matrix, row + delta.0, col + delta.1) {
                // change direction
                delta_idx = (delta_idx + 1) % deltas.len();
                delta = deltas[delta_idx];
            }
            (row, col) = (row + delta.0, col + delta.1);
        }
        count
    }

    pub fn parse_matrix(map: String) -> (Vec<Vec<char>>, (isize, isize)) {
        let mut start_pos = (0, 0);
        let matrix = map
            .lines()
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('^') {
                    start_pos = (i as isize, j as isize);
                };
                line.chars().collect()
            })
            .collect::<Vec<Vec<char>>>();
        (matrix, start_pos)
    }

    fn try_get_char(matrix: &Vec<Vec<char>>, row: isize, col: isize) -> Option<char> {
        matrix
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .copied()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day6::part1::{self, parse_matrix};

    #[test]
    pub fn test_file_1() {
        let map = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (matrix, start_pos) = parse_matrix(map.to_string());
        let count = part1::count_moves(matrix, start_pos);
        assert_eq!(count, 41);
    }

    #[test]
    pub fn run_part_1() {
        let count = part1::run("./day6.txt");
        assert_eq!(count, 5516);
    }
}
