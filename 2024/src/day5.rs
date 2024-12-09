use std::collections::HashMap;

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

pub mod part1 {

    use std::fs;

    use crate::day5::{is_update_valid, parse_puzzle};

    pub fn run(file_path: &str) -> i32 {
        let puzzle = fs::read_to_string(file_path).unwrap();
        let (rules, updates) = parse_puzzle(&puzzle);

        let count = updates
            .iter()
            .filter(|updates| is_update_valid(&rules, updates))
            .map(|updates| updates[updates.len() / 2])
            .sum();

        println!("count is {}", count);
        return count;
    }
}

pub mod part2 {
    use std::{collections::HashMap, fs};

    use crate::day5::{is_update_valid, parse_puzzle};

    pub fn run(file_path: &str) -> i32 {
        let puzzle = fs::read_to_string(file_path).unwrap();
        let (rules, mut updates) = parse_puzzle(&puzzle);

        let count = updates
            .iter_mut()
            .filter(|updates| !is_update_valid(&rules, updates))
            .map(|updates| sort_by_rules(&rules, updates))
            .map(|updates| updates[updates.len() / 2])
            .sum();

        println!("count is {}", count);
        return count;
    }

    pub fn sort_by_rules(rules: &HashMap<i32, Vec<i32>>, updates: &mut Vec<i32>) -> Vec<i32> {
        let mut swap_performed = true;
        while swap_performed {
            swap_performed = false;
            let mut seen = HashMap::new();
            for i in 0..updates.len() {
                let entry = updates[i];
                if let Some(must_be_after) = rules.get(&entry) {
                    if let Some(&j) = must_be_after.iter().find_map(|value| seen.get(value)) {
                        updates.swap(i, j);
                        swap_performed = true;
                    }
                }
                seen.entry(entry).or_insert(i);
            }
        }
        updates.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{parse_puzzle, part1, part2};

    #[test]
    pub fn test_parse_puzzle() {
        let input = "\
12|34

12,34,56,78";
        let (rules, lines) = parse_puzzle(input);
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

    #[test]
    pub fn test_sorting() {
        let tests = vec![
            (
                vec![(97, vec![75])],
                vec![75, 97, 47, 61, 53],
                vec![97, 75, 47, 61, 53],
            ),
            (
                vec![(2, vec![5, 3]), (3, vec![5])],
                vec![5, 3, 2],
                vec![2, 3, 5],
            ),
        ];
        for (hash, mut updates, expected) in tests {
            let mut rules = HashMap::new();
            for (before, after) in hash {
                rules.insert(before, after);
            }
            let new_list = part2::sort_by_rules(&rules, &mut updates);
            assert_eq!(new_list, expected);
        }
    }

    #[test]
    pub fn test_file2() {
        let count = part2::run("./day5.txt");
        assert_eq!(count, 6257, "got {} expected {}", count, 6257);
    }
}
