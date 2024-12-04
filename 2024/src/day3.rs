use std::{
    fs,
    io::{BufRead, BufReader},
};

pub fn parse_file(file_path: &str) -> Vec<(i32, i32)> {
    let file = fs::File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let nums_to_multiply: Vec<(i32, i32)> = Vec::new();
    for line in buf.lines() {
        println!("line: {:?}", line);
    }
    nums_to_multiply
}
pub mod part1 {
    use super::parse_file;

    pub fn run(file_path: &str) -> i32 {
        println!("running day 3 part1");
        let nums_to_multiply = parse_file(file_path);
        let mut total = 0;
        for (num1, num2) in nums_to_multiply {
            total += num1 * num2;
        }
        println!("total is: {}", total);
        total
    }
}

#[cfg(test)]
pub mod tests {
    use super::part1;

    #[test]
    fn part_1_test_file() {
        let expected = 161;
        let total = part1::run("./day3.test.txt");
        assert_eq!(total, expected, "got {} but expected {}", total, expected);
    }
}
