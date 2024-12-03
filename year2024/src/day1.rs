use std::fs;

struct IdLists {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl IdLists {
    pub fn new() -> Self {
        IdLists {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let ids: Vec<i32> = line
            .split_whitespace()
            .map(|id| id.parse::<i32>().expect("invalid formatting for line"))
            .collect();
        self.left.push(ids[0]);
        self.right.push(ids[1]);
    }
}

fn read_file(file_path: &str) -> IdLists {
    let file_contents = fs::read_to_string(file_path).expect("invalid input file");
    let mut id_lists = IdLists::new();
    for line in file_contents.lines() {
        id_lists.parse_line(line);
    }
    id_lists
}

pub mod part1 {
    use super::{read_file, IdLists};

    pub fn run(file_path: &str) -> i32 {
        println!("running day1 - part1");
        let IdLists { left, right } = read_file(file_path);
        let distance = calculate_distance(left, right);
        println!("distance: {}", distance);
        distance
    }

    fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
        left.sort();
        right.sort();
        let mut distance = 0;
        for i in 0..left.len() {
            distance += (left[i] - right[i]).abs();
        }
        distance
    }
}

pub mod part2 {
    use super::{read_file, IdLists};
    use std::collections::HashMap;

    pub fn run(file_path: &str) -> i32 {
        println!("running day1 - part2");
        let IdLists { left, right } = read_file(file_path);
        let similarity_score = calculate_similarity_score(left, right);
        println!("similarity_score: {}", similarity_score);
        similarity_score
    }

    fn calculate_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut similarity_score = 0;
        let mut right_counts: HashMap<i32, i32> = HashMap::new();
        for right in right {
            right_counts
                .entry(right)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for left in left {
            if let Some(right) = right_counts.get(&left) {
                similarity_score += right * left;
            }
        }
        similarity_score
    }
}
