pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> i32 {
        let map = fs::read_to_string(file_path).unwrap();
        let count = count_moves(&map);
        println!("count is: {}", count);
        count
    }

    #[derive(Debug)]
    pub struct Coord {
        row: isize,
        col: isize,
    }

    pub fn parse_matrix(map: &str) -> (Vec<Vec<char>>, Coord) {
        let mut start = Coord { row: 0, col: 0 };
        let matrix = map
            .lines()
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('^') {
                    start = Coord {
                        row: i as isize,
                        col: j as isize,
                    };
                };
                line.chars().collect()
            })
            .collect::<Vec<Vec<char>>>();
        (matrix, start)
    }

    pub fn count_moves(map: &str) -> i32 {
        let (mut matrix, mut coord) = parse_matrix(map);
        let max = matrix.len() as isize;

        let mut count = 0;
        let deltas: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut delta_idx = 0;
        while coord.row < max && coord.row >= 0 && coord.col < max && coord.col >= 0 {
            if matrix[coord.row as usize][coord.col as usize] != 'X' {
                count += 1;
            }
            matrix[coord.row as usize][coord.col as usize] = 'X';

            let mut delta = deltas[delta_idx];
            let (next_row, next_col) = (coord.row + delta.0, coord.col + delta.1);
            if let Some('#') = try_get_char(&matrix, next_row, next_col) {
                delta_idx += 1;
                if delta_idx >= deltas.len() {
                    delta_idx = 0;
                }
                delta = deltas[delta_idx];
            }

            coord.row += delta.0;
            coord.col += delta.1;
        }
        count
    }

    pub fn try_get_char(matrix: &Vec<Vec<char>>, row: isize, col: isize) -> Option<char> {
        matrix
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .copied()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day6::part1;

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
        let count = part1::count_moves(map);
        assert_eq!(count, 41);
    }

    #[test]
    pub fn run_part_1() {
        let count = part1::run("./day6.txt");
        assert_eq!(count, 5516);
    }
}
