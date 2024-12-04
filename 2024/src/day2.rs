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

fn get_safe_count(reports: Vec<Vec<i32>>, max_bad_levels: i32) -> i32 {
    let mut safe_count = 0;
    for level in reports.iter() {
        let safe = check_if_safe(level, max_bad_levels);
        if safe {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn check_if_safe(level: &Vec<i32>, max_bad_levels: i32) -> bool {
    let mut previous_diff: i32 = 0;
    let mut left = 0;
    let mut unsafe_count = 0;
    for right in 1..level.len() {
        let diff = level[right] - level[left];
        if (diff.abs() > 3 || diff.abs() < 1)
            || (diff < 0 && previous_diff > 0 || diff > 0 && previous_diff < 0)
        {
            unsafe_count += 1;
            if unsafe_count > max_bad_levels {
                return false;
            }
        } else {
            left = right;
            previous_diff = diff;
        }
    }
    return true;
}

pub mod part1 {
    use super::*;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part1");
        let reports = parse_file(file_path);
        let safe_count = get_safe_count(reports, 0);
        println!("safe count: {}", safe_count);
        safe_count
    }
}

pub mod part2 {
    use super::*;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part2");
        let reports = parse_file(file_path);
        let safe_count = get_safe_count(reports, 1);
        println!("safe count: {}", safe_count);
        safe_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // pub fn run_part_2() {
    //     let expected = 572;
    //     let result = part2::run("./day2.txt");
    //     assert_eq!(expected, result);
    // }

    #[test]
    pub fn test_check_if_safe() {
        let tests = vec![
            (vec![7, 6, 4, 2, 1], true, 0),
            (vec![1, 2, 7, 8, 9], false, 0),
            (vec![1, 3, 2, 4, 5], true, 1),
            (vec![1, 2, 7, 8, 9], false, 1),
            (vec![8, 6, 4, 4, 1], true, 1),
        ];
        for (input, expected, max_bad_levels) in tests {
            let result = check_if_safe(&input, max_bad_levels);
            assert_eq!(
                result, expected,
                "failed with:{:?} expected:{} max_bad_levels: {}",
                input, expected, max_bad_levels
            );
        }
    }
}
