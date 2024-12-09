pub mod part1 {

    use std::{collections::HashMap, fs};

    pub fn run(file_path: &str) -> i32 {
        let puzzle = fs::read_to_string(file_path).unwrap();
        let (rules, updates_list) = parse_puzzle(&puzzle);

        let count = updates_list
            .iter()
            .filter(|updates| is_update_valid(&rules, updates))
            .map(|updates| updates[updates.len() / 2])
            .sum();

        println!("count is {}", count);
        return count;
    }

    pub fn is_update_valid(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<i32>) -> bool {
        let mut seen = HashMap::new();
        for entry in updates {
            if let Some(must_be_after) = rules.get(entry) {
                if must_be_after.iter().any(|value| seen.contains_key(value)) {
                    return false;
                }
            }
            seen.entry(entry).or_insert(true);
        }
        true
    }

    pub fn parse_puzzle(puzzle: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
        let sections: Vec<&str> = puzzle.split("\n\n").collect();
        (
            sections[0]
                .lines()
                .map(|line| line.split("|").map(|s| s.parse::<i32>().unwrap()).collect())
                .fold(HashMap::new(), |mut rules, nums: Vec<i32>| {
                    rules
                        .entry(nums[0])
                        .and_modify(|rules| rules.push(nums[1]))
                        .or_insert(vec![nums[1]]);
                    rules
                }),
            sections[1]
                .lines()
                .map(|line| line.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
                .fold(Vec::new(), |mut updates, nums: Vec<i32>| {
                    updates.push(nums);
                    updates
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    pub fn test_parse_puzzle() {
        let input = "\
12|34

12,34,56,78";
        let (rules, lines) = part1::parse_puzzle(input);
        assert_eq!(rules.get(&12).unwrap()[0], 34);
        assert_eq!(lines, vec![vec![12, 34, 56, 78]]);
    }

    #[test]
    pub fn test_file() {
        let count = part1::run("./day5.test.txt");
        assert_eq!(count, 143, "got {} expected {}", count, 143);
        let count = part1::run("./day5.txt");
        assert_eq!(count, 4766, "got {} expected {}", count, 4766);
    }
}
