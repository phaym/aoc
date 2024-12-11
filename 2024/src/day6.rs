use std::collections::HashSet;

pub fn traverse_map(
    matrix: &Vec<Vec<char>>,
    (mut row, mut col): (isize, isize),
) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let max = matrix.len() as isize;

    let mut visited = HashSet::new();
    let mut obstacles = Vec::new();
    let deltas: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut delta_idx = 0;
    while row < max && row >= 0 && col < max && col >= 0 {
        visited.insert((row as usize, col as usize));
        let mut delta = deltas[delta_idx];
        let (next_row, next_col) = (row + delta.0, col + delta.1);
        if let Some('#') = try_get_char(&matrix, next_row, next_col) {
            // change direction
            obstacles.push((next_row as usize, next_col as usize));
            delta_idx = (delta_idx + 1) % deltas.len();
            delta = deltas[delta_idx];
        }
        (row, col) = (row + delta.0, col + delta.1);
    }
    (visited, obstacles)
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

pub mod part1 {

    use std::fs;

    use super::{parse_matrix, traverse_map};

    pub fn run(file_path: &str) -> usize {
        let map = fs::read_to_string(file_path).unwrap();
        let (matrix, start_pos) = parse_matrix(map);
        let (visited, _) = traverse_map(&matrix, start_pos);
        println!("count is: {}", visited.len());
        visited.len()
    }
}

pub mod part2 {
    use std::fs;

    use super::{parse_matrix, traverse_map};

    pub fn run(file_path: &str) -> i32 {
        println!("starting day6 part 2");
        let map = fs::read_to_string(file_path).unwrap();
        let result = part_2(map);
        println!("count: {}", result);
        result
    }

    pub fn part_2(map: String) -> i32 {
        let (matrix, start_pos) = parse_matrix(map);
        let (_visited, obstacles) = traverse_map(&matrix, start_pos);

        let mut vertical = false;
        let mut count = 0;
        for i in 2..obstacles.len() - 1 {
            let last = obstacles[i];
            let next = obstacles[i + 1];
            for j in (0..i).rev() {
                let curr = obstacles[j];
                if !vertical {
                    let right = next.0 > last.0;
                    if (right && curr.1 > last.1 && curr.0 < next.0)
                        || (curr.1 < last.1 && curr.0 > next.0)
                    {
                        println!(
                            "yes horizontal i{} j{} last{:?} next{:?} curr{:?}",
                            i, j, last, next, curr
                        );
                        count += 1;
                        break;
                    }
                } else if (curr.0 < last.0 && next.0 < curr.0)
                    || (curr.0 > last.0 && next.0 > curr.0)
                {
                    println!(
                        "yes vertical i{} j{} last{:?} next{:?} curr{:?}",
                        i, j, last, next, curr
                    );
                    count += 1;
                    break;
                }
            }
            vertical = !vertical;
        }
        // let mut queue = VecDeque::from(obstacles);
        // while queue.len() > 3 {
        //     let first = queue[0];
        //     let last = queue[2];
        //     let next = queue[3];
        //     if !vertical
        //         && ((first.1 < last.1 && next.1 < first.1)
        //             || (first.1 > last.1 && next.1 > first.1))
        //     {
        //         println!("yes horizontal");
        //         count += 1;
        //     } else if vertical
        //         && ((first.0 < last.0 && next.0 < first.0)
        //             || (first.0 > last.0 && next.0 > first.0))
        //     {
        //         println!("yes vertical");
        //         count += 1;
        //     }
        //     for (row, col) in queue.range(0..4) {
        //         println!("row {}, col {}", row, col);
        //     }
        //     queue.pop_front();
        //     vertical = !vertical;
        //     println!("next");
        // }
        count
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day6::{parse_matrix, part1, part2::part_2, traverse_map};

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
        let (visited, _) = traverse_map(&matrix, start_pos);
        assert_eq!(visited.len(), 41);
    }

    #[test]
    pub fn run_part_1() {
        let count = part1::run("./day6.txt");
        assert_eq!(count, 5516);
    }

    #[test]
    pub fn run_test_file_2() {
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
        let result = part_2(map.to_string());
        assert_eq!(result, 6);
    }

    #[test]
    pub fn maps() {
        let map = "\
#...
...#
....
^.#.";
        let result = part_2(map.to_string());
        assert_eq!(result, 1);
    }
}
