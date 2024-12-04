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
    use super::*;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part1");
        let reports = parse_file(file_path);
        let mut safe_count = 0;
        for level in reports.iter() {
            let safe = check_if_safe(level);
            if safe {
                safe_count += 1;
            }
        }
        println!("safe count: {}", safe_count);
        safe_count
    }

    pub fn check_if_safe(level: &Vec<i32>) -> bool {
        let mut diff: i32 = 0;
        for i in 1..level.len() {
            let current_diff = level[i] - level[i - 1];
            if current_diff.abs() > 3 || current_diff.abs() < 1 {
                println!("unsafe diff of {} at i:{}", current_diff, i);
                return false;
            } else if current_diff < 0 && diff > 0 || current_diff > 0 && diff < 0 {
                println!("changed direction at i:{}", i);
                return false;
            }
            diff = current_diff;
        }
        return true;
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
    pub fn test_file() {
        let expected = 2;
        let result = part1::run("./day2.test.txt");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn part_1() {
        let expected = 572;
        let result = part1::run("./day2.txt");
        assert_eq!(expected, result);
    }
}
