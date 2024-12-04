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

fn get_safe_count_2(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for level in reports.iter() {
        let safe = check_if_safe_2(level);
        if safe {
            safe_count += 1;
        }
    }
    safe_count
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
        let has_direction_change = diff < 0 && previous_diff > 0 || diff > 0 && previous_diff < 0;
        let has_invalid_diff = diff.abs() > 3 || diff.abs() < 1;
        if has_direction_change || has_invalid_diff {
            unsafe_count += 1;
            if unsafe_count > max_bad_levels {
                return false;
            } else if has_invalid_diff {
            }
        } else {
            left = right;
            previous_diff = diff;
        }
    }
    return true;
}

pub fn validate_diff(current: i32, next: i32) -> bool {
    (current.abs() > 3 || current.abs() < 1) || (current < 0 && next > 0 || current > 0 && next < 0)
}

pub fn check_if_safe_2(level: &Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = Vec::new();
    for i in 1..level.len() {
        diffs.push(level[i] - level[i - 1]);
    }
    println!("starting diffs for {:?} are {:?}", level, diffs);
    let mut unsafe_count = 0;
    for i in 0..diffs.len() - 1 {
        if !validate_diff(diffs[i], diffs[i + 1]) {
            unsafe_count += 1;
            if unsafe_count > 1 {
                println!("removed more then 1 level");
                return false;
            }
            if i < diffs.len() - 1 && validate_diff(diffs[i] + diffs[i + 1], diffs[i + 2]) {
                diffs[i + 1] = diffs[i] + diffs[i + 1];
            }
            diffs[i] = 0;
        }
    }
    // println!("after abs diffs for {:?} are {:?}", level, diffs);
    // println!("unsafe_count: {} ", unsafe_count);
    // for i in 0..diffs.len() - 1 {
    //     if diffs[i] < 0 && diffs[i + 1] > 0 || diffs[i] > 0 && diffs[i + 1] < 0 {
    //         unsafe_count += 1;
    //         if unsafe_count > 1 {
    //             return false;
    //         }
    //         diffs[i + 1] = diffs[i] + diffs[i + 1];
    //         diffs[i] = 0;
    //     };
    // }
    println!("diffs for {:?} are {:?}", level, diffs);
    println!("unsafe_count: {} ", unsafe_count);
    if unsafe_count > 1 {
        return false;
    } else {
        return true;
    }
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
        let safe_count = get_safe_count_2(reports);
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
    pub fn test_check_special() {
        let tests = vec![
            (vec![1, 5, 2, 4, 5], true, 0),      // remove 5
            (vec![40, 50, 38, 36, 34], true, 0), // remove 50
            (vec![40, 50, 43, 46, 47], true, 0), // remove 40
            (vec![10, 12, 8, 6], true, 0),       // remove 40
            (vec![10, 52, 53, 54], true, 0),     // remove 40
        ];
        for (input, expected, max_bad_levels) in tests {
            let result = check_if_safe_2(&input);
            assert_eq!(
                result, expected,
                "failed with:{:?} expected:{} max_bad_levels: {}",
                input, expected, max_bad_levels
            );
        }
    }

    #[test]
    pub fn test_check_if_safe() {
        let tests = vec![
            // (vec![7, 6, 4, 2, 1], true, 0),
            // (vec![1, 2, 7, 8, 9], false, 0),
            (vec![1, 3, 2, 4, 5], true, 1),
            (vec![1, 2, 7, 8, 9], false, 1), //cant remove any
            (vec![8, 6, 4, 4, 1], true, 1),
            (vec![8, 6, 9, 4, 5], false, 1),
            (vec![9, 3, 5, 7, 8], true, 1),  // remove 0 for decreasing
            (vec![1, 3, 5, 7, 3], true, 1),  // remove 4
            (vec![1, 5, 3, 2, 1], true, 1),  // remove 0 for increasing
            (vec![10, 5, 3, 2, 1], true, 1), // remove 0 for decreasing too much
            (vec![1, 5, 6, 7, 8], true, 1),  // remove 0 for increasing too much
            (vec![1, 5, 2, 4, 5], true, 1),  // remove 1
        ];
        for (input, expected, max_bad_levels) in tests {
            let result = check_if_safe_2(&input);
            assert_eq!(
                result, expected,
                "failed with:{:?} expected:{} max_bad_levels: {}",
                input, expected, max_bad_levels
            );
        }
    }
}
