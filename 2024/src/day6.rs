pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> i32 {
        let map = fs::read_to_string(file_path).unwrap();
        let count = count_moves(&map);
        println!("count is: {}", count);
        count
    }

    pub fn parse_matrix(map: &str) -> (Vec<Vec<char>>, (isize, isize)) {
        let mut start = (0, 0);
        let matrix = map
            .lines()
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('^') {
                    start = (i as isize, j as isize);
                };
                line.chars().collect()
            })
            .collect::<Vec<Vec<char>>>();
        (matrix, start)
    }

    pub fn count_moves(map: &str) -> i32 {
        let (mut matrix, mut pos) = parse_matrix(map);
        let max = matrix.len() as isize;

        let mut count = 0;
        let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut dir_idx = 0;
        while pos.0 < max && pos.0 >= 0 && pos.1 < max && pos.1 >= 0 {
            if matrix[pos.0 as usize][pos.1 as usize] != 'X' {
                count += 1;
            }
            matrix[pos.0 as usize][pos.1 as usize] = 'X';

            let mut next = (pos.0 + dirs[dir_idx].0, pos.1 + dirs[dir_idx].1);
            if matrix
                .get(next.0 as usize)
                .and_then(|row| row.get(next.1 as usize))
                .copied()
                .unwrap_or('a')
                == '#'
            {
                dir_idx += 1;
                if dir_idx >= dirs.len() {
                    dir_idx = 0;
                }
                next = (pos.0 + dirs[dir_idx].0, pos.1 + dirs[dir_idx].1);
            }

            pos.0 = next.0;
            pos.1 = next.1;
        }
        count
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
