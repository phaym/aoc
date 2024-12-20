use std::{collections::HashMap, fs};

pub fn stone_count(stone: u64, blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&result) = memo.get(&(stone, blinks)) {
        return result;
    }
    if blinks == 0 {
        return 1;
    }

    let stone_str = stone.to_string();
    let result = if stone == 0 {
        stone_count(1, blinks - 1, memo)
    } else if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone.to_string().len() / 2);
        stone_count(left.parse().unwrap(), blinks - 1, memo)
            + stone_count(right.parse().unwrap(), blinks - 1, memo)
    } else {
        stone_count(stone * 2024, blinks - 1, memo)
    };

    memo.insert((stone, blinks), result);
    result
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

    use crate::day11::{parse_file, stone_count};

    pub fn run(file_path: &str) -> u64 {
        let parsed = parse_file(file_path);
        let mut result = 0;
        let mut memo = HashMap::new();
        for stone in parsed {
            result += stone_count(stone, 25, &mut memo);
        }
        println!("result: {}", result);
        result
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::day11::{parse_file, stone_count};

    pub fn run(file_path: &str) -> u64 {
        let parsed = parse_file(file_path);
        let mut result = 0;
        let mut memo = HashMap::new();
        for stone in parsed {
            result += stone_count(stone, 75, &mut memo);
        }
        println!("result: {}", result);
        result
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::day11::{part1::run, stone_count};

    #[test]
    pub fn test_blink() {
        let tests = vec![
            (125, 1, 1),
            (125, 2, 2),
            (125, 3, 2),
            (125, 4, 3),
            (125, 5, 5),
            (125, 6, 7),
            (0, 25, 19778),
        ];
        for (input, blinks, expected) in tests {
            let mut memo = HashMap::new();
            let result = stone_count(input, blinks, &mut memo);
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
