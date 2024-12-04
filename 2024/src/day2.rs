use std::fs;

fn parse_line(line: &str) -> Vec<i32> {
    let ids: Vec<i32> = line
        .split_whitespace()
        .map(|id| id.parse::<i32>().expect("invalid formatting for line"))
        .collect();
    ids
}

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let file_contents = fs::read_to_string(file_path).expect("invalid input file");
    let mut reports = Vec::new();
    for line in file_contents.lines() {
        let levels = parse_line(line);
        reports.push(levels);
    }
    reports
}

pub mod part1 {
    use super::parse_file;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part1");
        let reports = parse_file(file_path);
        let safe_count = get_safe_count(reports);
        println!("safe count: {}", safe_count);
        safe_count
    }

    fn get_safe_count(reports: Vec<Vec<i32>>) -> i32 {
        let mut safe_count = 0;
        for level in reports.iter() {
            let safe = check_if_safe(level);
            if safe {
                safe_count += 1;
            }
        }
        safe_count
    }
    pub fn check_if_safe(level: &Vec<i32>) -> bool {
        let mut previous_diff: i32 = 0;
        let mut left = 0;
        for right in 1..level.len() {
            let diff = level[right] - level[left];
            let has_direction_change =
                diff < 0 && previous_diff > 0 || diff > 0 && previous_diff < 0;
            let has_invalid_diff = diff.abs() > 3 || diff.abs() < 1;
            if has_direction_change || has_invalid_diff {
                return false;
            } else {
                left = right;
                previous_diff = diff;
            }
        }
        return true;
    }
}

pub mod part2 {
    use super::parse_file;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part2");
        let reports = parse_file(file_path);
        let safe_count = get_safe_count(reports);
        println!("safe count: {}", safe_count);
        safe_count
    }

    fn get_safe_count(reports: Vec<Vec<i32>>) -> i32 {
        let mut safe_count = 0;
        for level in reports.iter() {
            let safe = check_if_safe(level);
            if safe {
                safe_count += 1;
            }
        }
        safe_count
    }

    pub fn is_invalid_diff(diff: i32, expect_positive: bool) -> bool {
        (diff.abs() > 3 || diff.abs() < 1)
            || (expect_positive && diff < 0 || !expect_positive && diff > 0)
    }

    pub fn check_if_safe(level: &Vec<i32>) -> bool {
        let mut diffs: Vec<i32> = Vec::new();
        let mut pos_count: i32 = 0;
        let mut neg_count: i32 = 0;
        for i in 1..level.len() {
            let diff = level[i] - level[i - 1];
            diffs.push(diff);
            if diff > 0 {
                pos_count += 1;
            } else if diff < 0 {
                neg_count += 1;
            }
        }
        println!(
            "starting diffs for {:?} are {:?} pos_count:{}, neg_count:{}",
            level, diffs, pos_count, neg_count
        );
        if pos_count > 1 && neg_count > 1 {
            println!("too many direction changes, unsafe");
            return false;
        }
        let is_positive = pos_count > neg_count;
        let mut unsafe_count = 0;
        for i in 0..diffs.len() {
            let current_diff = diffs[i];
            if is_invalid_diff(current_diff, is_positive) {
                unsafe_count += 1;
                println!(
                    "current invalid_diff diff {} i:{} len:{}",
                    current_diff,
                    i,
                    diffs.len()
                );
                if unsafe_count > 1 {
                    println!("unsafe count exceeded, unsafe");
                    return false;
                } else if unsafe_count == 1 && i == diffs.len() - 1 {
                    println!("only last level is unsafe, safe!");
                    return true;
                }
                if i < diffs.len() - 1 && !is_invalid_diff(current_diff + diffs[i + 1], is_positive)
                {
                    diffs[i + 1] += current_diff;
                } else if i > 0 && is_invalid_diff(current_diff + diffs[i - 1], is_positive) {
                    println!("could not be fixed, unsafe");
                    return false;
                }
                diffs[i] = 0;
            }
        }
        println!("reached end, safe");
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_line, part1, part2};

    #[test]
    pub fn parses_line() {
        let test = "7 6 4 2 1";
        let expected = vec![7, 6, 4, 2, 1];
        let result = parse_line(test);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn run_part_1_test_file() {
        let expected = 2;
        let result = part1::run("./day2.test.txt");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn run_part_1() {
        let expected = 572;
        let result = part1::run("./day2.txt");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn run_part_2_test_file() {
        let expected = 4;
        let result = part2::run("./day2.test.txt");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn run_part_2() {
        let expected = 612;
        let result = part2::run("./day2.txt");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_check_if_safe() {
        let tests = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 3, 2, 4, 5], true),
            (vec![1, 2, 7, 8, 9], false), //cant remove any
            (vec![8, 6, 4, 4, 1], true),
            (vec![8, 6, 9, 4, 5], false),
            (vec![9, 3, 5, 7, 8], true),      // remove 0 for decreasing
            (vec![1, 3, 5, 7, 3], true),      // remove 4
            (vec![1, 5, 3, 2, 1], true),      // remove 0 for decreasing
            (vec![10, 5, 3, 2, 1], true),     // remove 0 for decreasing too much
            (vec![1, 5, 6, 7, 8], true),      // remove 0 for increasing too much
            (vec![1, 5, 2, 4, 5], true),      // remove 1
            (vec![1, 5, 2, 4, 5], true),      // remove 5
            (vec![40, 50, 38, 36, 34], true), // remove 50
            (vec![40, 50, 43, 46, 47], true), // remove 40
            (vec![10, 12, 8, 6], true),       // remove 40
            (vec![10, 52, 53, 54], true),     // remove 40
            (vec![], true),
            (vec![1, 2, 2, 4, 2], false), // remove 40
        ];
        for (input, expected) in tests {
            let result = part2::check_if_safe(&input);
            assert_eq!(
                result, expected,
                "failed with:{:?} expected:{} ",
                input, expected,
            );
        }
    }
}
