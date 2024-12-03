use std::fs;

struct Ids {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Ids {
    pub fn new() -> Self {
        Ids {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

fn parse_line(line: &str) -> (i32, i32) {
    let ids: Vec<i32> = line
        .split_whitespace()
        .map(|id| id.parse::<i32>().expect("invalid formatting for line"))
        .collect();
    (ids[0], ids[1])
}

fn parse_file(file_path: &str) -> Ids {
    let file_contents = fs::read_to_string(file_path).expect("invalid input file");
    let mut ids = Ids::new();
    for line in file_contents.lines() {
        let (left, right) = parse_line(line);
        ids.left.push(left);
        ids.right.push(right);
    }
    ids
}

pub mod part1 {
    use super::{parse_file, Ids};

    pub fn run(file_path: &str) -> i32 {
        println!("running day1 - part1");
        let ids = parse_file(file_path);
        let distance = calculate_distance(ids);
        println!("distance: {}", distance);
        distance
    }

    fn calculate_distance(mut ids: Ids) -> i32 {
        ids.left.sort();
        ids.right.sort();
        let mut distance = 0;
        for i in 0..ids.left.len() {
            distance += (ids.left[i] - ids.right[i]).abs();
        }
        distance
    }
}

pub mod part2 {
    use super::{parse_file, Ids};
    use std::collections::HashMap;

    pub fn run(file_path: &str) -> i32 {
        println!("running day1 - part2");
        let ids = parse_file(file_path);
        let similarity_score = calculate_similarity_score(ids);
        println!("similarity_score: {}", similarity_score);
        similarity_score
    }

    fn calculate_similarity_score(ids: Ids) -> i32 {
        let mut similarity_score = 0;
        let mut right_counts: HashMap<i32, i32> = HashMap::new();
        for right in ids.right {
            right_counts
                .entry(right)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for left in ids.left {
            if let Some(right) = right_counts.get(&left) {
                similarity_score += right * left;
            }
        }
        similarity_score
    }
}
