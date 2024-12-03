use std::fs;

pub fn run(file_path: &str) -> i32 {
    println!("running day1 - part2");
    let file_contents = fs::read_to_string(file_path).expect("invalid input file");
    let (left, right) = parse_id_lists(file_contents);
    let similarity_score = calculate_similarity_score(left, right);
    println!("similarity_score: {}", similarity_score);
    similarity_score
}

fn calculate_similarity_score(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    let mut distance = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    distance
}

fn parse_id_lists(file_contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in file_contents.lines() {
        let ids: Vec<i32> = line
            .split_whitespace()
            .map(|id| id.parse::<i32>().expect("invalid formatting for line"))
            .collect();
        left.push(ids[0]);
        right.push(ids[1]);
    }
    (left, right)
}
