use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").expect("invalid input file");
    let (left, right) = parse_to_lists(file_contents);
    println!("left:{:?}", left);
    println!("right:{:?}", right);
}

fn parse_to_lists(file_contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in file_contents.lines() {
        let mut split = line.split_whitespace();
        let location_id1 = split.next().unwrap();
        let location_id2 = split.next().unwrap();
        left.push(location_id1.parse::<i32>().unwrap());
        right.push(location_id2.parse::<i32>().unwrap());
    }
    (left, right)
}
