pub mod part1 {
    use std::{fs, io::BufRead, io::BufReader};

    use regex::Regex;

    pub fn run(file_path: &str) -> i32 {
        println!("running day 3 part1");
        let mut total = 0;
        for (num1, num2) in parse_file(file_path) {
            total += num1 * num2;
        }
        println!("total is: {}", total);
        total
    }

    pub fn parse_file(file_path: &str) -> Vec<(i32, i32)> {
        let file = fs::File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut nums_to_multiply: Vec<(i32, i32)> = Vec::new();

        for line in buf.lines().map(|l| l.unwrap()) {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            for (_, [num1, num2]) in re.captures_iter(&line).map(|c| c.extract()) {
                let values = (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap());
                nums_to_multiply.push(values);
            }
        }

        nums_to_multiply
    }
}

pub mod part2 {
    use std::{
        fs,
        io::{BufRead, BufReader},
    };

    use regex::Regex;

    pub fn parse_file(file_path: &str) -> Vec<(i32, i32)> {
        let file = fs::File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut nums_to_multiply: Vec<(i32, i32)> = Vec::new();
        let mut enabled = true;

        for line in buf.lines().map(|l| l.unwrap()) {
            let (result, is_enabled) = parse_line(&line, enabled);
            enabled = is_enabled;
            nums_to_multiply.extend(result);
        }

        nums_to_multiply
    }

    pub fn parse_line(line: &str, mut enabled: bool) -> (Vec<(i32, i32)>, bool) {
        let mut nums_to_multiply: Vec<(i32, i32)> = Vec::new();
        let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

        for cap in re.captures_iter(line) {
            match cap.get(0).unwrap().as_str() {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {
                    if enabled {
                        let num1 = cap.get(1).unwrap().as_str();
                        let num2 = cap.get(2).unwrap().as_str();
                        let values = (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap());

                        nums_to_multiply.push(values);
                    }
                }
            };
        }
        (nums_to_multiply, enabled)
    }

    pub fn run(file_path: &str) -> i32 {
        println!("running day3 part 2");
        let mut total = 0;
        for (num1, num2) in parse_file(file_path) {
            total += num1 * num2;
        }
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
        let expected = vec![(2, 4), (8, 5)];
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (result, _) = part2::parse_line(input, true);
        assert_eq!(
            result, expected,
            "got {:?} but expected {:?}",
            result, expected
        );
    }
}
