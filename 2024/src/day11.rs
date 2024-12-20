use std::{collections::HashMap, fs};

pub fn map_stone(stone: u64, count: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if memo.contains_key(&(stone, count)) {
        return *memo.get(&(stone, count)).unwrap();
    }
    if count == 0 {
        return 1;
    }
    let stone_str = stone.to_string();
    if stone == 0 {
        return map_stone(1, count - 1, memo);
    } else if stone_str.len() % 2 == 0 {
        let (first, last) = stone_str.split_at(stone.to_string().len() / 2);
        let (first_int, last_int) = (first.parse().unwrap(), last.parse().unwrap());
        let left = map_stone(first_int, count - 1, memo);
        memo.insert((first_int, count - 1), left);
        let right = map_stone(last_int, count - 1, memo);
        memo.insert((last_int, count - 1), right);
        return left + right;
    } else {
        return map_stone(stone * 2024, count - 1, memo);
    }
}

pub fn parse_file(file_path: &str) -> Vec<u64> {
    let file = fs::read_to_string(file_path).unwrap();
    file.trim()
        .split(' ')
        .map(|c| c.parse::<u64>().expect(&format!("{:?}", c)))
        .collect()
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::day11::{map_stone, parse_file};

    pub fn run(file_path: &str) -> u64 {
        let parsed = parse_file(file_path);
        let mut result = 0;
        let mut memo = HashMap::new();
        for val in parsed {
            result += map_stone(val, 25, &mut memo);
        }
        println!("result: {}", result);
        result
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::day11::{map_stone, parse_file};

    pub fn run(file_path: &str) -> u64 {
        let parsed = parse_file(file_path);
        let mut result = 0;
        let mut memo = HashMap::new();
        for val in parsed {
            result += map_stone(val, 75, &mut memo);
        }
        println!("result: {}", result);
        result
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::day11::{map_stone, part1::run};

    #[test]
    pub fn test_blink() {
        let tests = vec![
            (125, 1, 1),
            (125, 2, 2),
            (125, 3, 2),
            (125, 4, 3),
            (125, 5, 5),
            (125, 6, 7),
            (0, 25, 5442),
        ];
        for (input, blinks, expected) in tests {
            let mut memo = HashMap::new();
            let result = map_stone(input, blinks, &mut memo);
            println!("got result");
            assert_eq!(
                result, expected,
                "got {result} for {blinks} expected{expected}"
            );
        }
    }

    #[test]
    pub fn test_file_part1() {
        let result = run("./day11.test.txt");
        assert_eq!(result, 55312);
    }
}
