use std::fs;

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new() -> Self {
        Report { levels: Vec::new() }
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    let ids: Vec<i32> = line
        .split_whitespace()
        .map(|id| id.parse::<i32>().expect("invalid formatting for line"))
        .collect();
    ids
}

fn parse_file(file_path: &str) -> Vec<Report> {
    let file_contents = fs::read_to_string(file_path).expect("invalid input file");
    let mut reports: Vec<Report> = Vec::new();
    for line in file_contents.lines() {
        let mut report = Report::new();
        report.levels = parse_line(line);
        reports.push(report);
    }
    reports
}

pub mod part1 {
    use super::*;

    pub fn run(file_path: &str) -> i32 {
        println!("running day2 part1");
        let reports = parse_file(file_path);
        let mut safe_count = 0;
        for level in reports.iter().map(|report| &report.levels) {
            let mut diff: i32 = 0;
            let mut safe = true;
            for i in 1..level.len() {
                let current_diff = level[i] - level[i - 1];
                if current_diff.abs() > 3 || current_diff.abs() < 1 {
                    println!("unsafe diff of {} at i:{}", current_diff, i);
                    safe = false;
                    break;
                }
                if current_diff < 0 && diff > 0 || current_diff > 0 && diff < 0 {
                    println!("changed direction at i:{}", i);
                    safe = false;
                    break;
                }
                diff = current_diff;
            }
            if safe {
                safe_count += 1;
            }
        }
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
