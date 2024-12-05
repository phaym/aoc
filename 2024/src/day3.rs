pub mod part1 {
    use std::{fs, io::BufRead, io::BufReader};

    use regex::Regex;

    pub fn run(file_path: &str) -> i32 {
        println!("running day 3 part1");
        let total = parse_file(file_path).into_iter().sum();
        println!("total is: {}", total);
        total
    }

    pub fn parse_file(file_path: &str) -> Vec<i32> {
        let file = fs::File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut sums = Vec::new();

        for line in buf.lines().map(|l| l.unwrap()) {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            for (_, [num1, num2]) in re.captures_iter(&line).map(|c| c.extract()) {
                sums.push(num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap());
            }
        }
        sums
    }
}

pub mod part2 {
    use std::{
        fs,
        io::{BufRead, BufReader},
    };

    use regex::Regex;

    pub fn parse_file(file_path: &str) -> Vec<i32> {
        let file = fs::File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut enabled = true;

        buf.lines()
            .filter_map(Result::ok)
            .map(|line| {
                let (result, new_enabled) = parse_line(&line, enabled);
                enabled = new_enabled;
                result
            })
            .collect()
    }

    pub fn parse_line(line: &str, enabled: bool) -> (i32, bool) {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
        let mut current_enabled = enabled;
        let mut sum = 0;

        for cap in re.captures_iter(line) {
            match cap.get(0).unwrap().as_str() {
                "don't()" => current_enabled = false,
                "do()" => current_enabled = true,
                _ if current_enabled => {
                    let num1: i32 = cap[1].parse().unwrap();
                    let num2: i32 = cap[2].parse().unwrap();
                    sum += num1 * num2;
                }
                _ => {}
            };
        }
        (sum, current_enabled)
    }

    pub fn run(file_path: &str) -> i32 {
        println!("running day3 part 2");
        let total = parse_file(file_path).into_iter().sum();
        println!("total is: {}", total);
        total
    }
}

#[cfg(test)]
pub mod tests {
    use super::part2;

    use super::part1;

    #[test]
    fn part_1_test_file() {
        let expected = 161;
        let total = part1::run("./day3.test.txt");
        assert_eq!(total, expected, "got {} but expected {}", total, expected);
    }

    #[test]
    fn part_2_test_file() {
        let expected = 89798695;
        let total = part2::run("./day3.txt");
        assert_eq!(total, expected, "got {} but expected {}", total, expected);
    }

    #[test]
    fn parse_line() {
        let expected = 48;
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (result, _) = part2::parse_line(input, true);
        assert_eq!(
            result, expected,
            "got {:?} but expected {:?}",
            result, expected
        );
    }
}
