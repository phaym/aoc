pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> usize {
        let file = fs::read_to_string(file_path).unwrap();

        let mut parsed = file
            .trim()
            .split(' ')
            .map(|c| c.parse::<i32>().expect(&format!("{:?}", c)))
            .collect();
        for _i in 0..25 {
            blink(&mut parsed);
        }
        parsed.len()
    }

    pub fn blink(stones: &mut Vec<i32>) {
        for i in 0..stones.len() {
            let stone = stones[i];
            let stone_str = stone.to_string();
            if stone == 0 {
                stones[i] = 1 as i32;
            } else if stone_str.len() % 2 == 0 {
                let (first, last) = stone_str.split_at(stone.to_string().len() / 2);
                stones[i] = first.parse::<i32>().unwrap();
                stones.insert(i + 1, last.parse::<i32>().unwrap());
            } else {
                stones[i] *= 2024;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day11::part1::{blink, run};

    #[test]
    pub fn test_blink() {
        let mut input = vec![125, 17];
        blink(&mut input);
        assert_eq!(input, vec![253000, 1, 7]);
    }

    #[test]
    pub fn test_file_part1() {
        let result = run("./day11.test.txt");
        assert_eq!(result, 55312);
    }
}
