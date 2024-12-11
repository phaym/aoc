pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> i64 {
        let puzzle = fs::read_to_string(file_path).unwrap();
        let total = get_total(puzzle);
        println!("got total: {}", total);
        total
    }

    pub fn get_total(puzzle: String) -> i64 {
        puzzle
            .lines()
            .map(|line| parse_line(line))
            .map(|(total, nums)| if is_valid(total, nums, 0) { total } else { 0 })
            .sum()
    }

    pub fn parse_line(line: &str) -> (i64, Vec<i64>) {
        let segments: Vec<&str> = line.split(':').collect();
        let total = segments[0].parse::<i64>().unwrap();
        let nums = segments[1]
            .trim()
            .split(' ')
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        (total, nums)
    }

    pub fn is_valid(total: i64, nums: Vec<i64>, current: i64) -> bool {
        // println!("total: {}, current{}, nums: {:?} ", total, current, nums);
        if nums.len() == 0 {
            return if total == current { true } else { false };
        }

        let sum_total = is_valid(total, nums[1..nums.len()].to_vec(), current + nums[0]);
        let multi_total = is_valid(
            total,
            nums[1..nums.len()].to_vec(),
            if current == 0 { 1 } else { current } * nums[0],
        );
        return sum_total || multi_total;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day7::part1::is_valid;

    use super::part1::get_total;

    #[test]
    pub fn test_file_1() {
        let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = get_total(input.to_string());
        assert_eq!(result, 3749);
    }
    #[test]
    pub fn check_is_valid() {
        let total = is_valid(190, vec![10, 19], 0);
        assert_eq!(total, true);
    }
    #[test]
    pub fn check_is_valid2() {
        let total = is_valid(3267, vec![81, 40, 27], 0);
        assert_eq!(total, true);
    }
    #[test]
    pub fn check_is_valid3() {
        let total = is_valid(156, vec![15, 6], 0);
        assert_eq!(total, false);
    }
}
