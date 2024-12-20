pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> usize {
        let file = fs::read_to_string(file_path).unwrap();

        let mut parsed = file
            .trim()
            .split(' ')
            .map(|c| c.parse::<u64>().expect(&format!("{:?}", c)))
            .collect();
        for _i in 0..25 {
            blink(&mut parsed);
        }
        println!("result: {}", parsed.len());
        parsed.len()
    }

    pub fn blink(stones: &mut Vec<u64>) {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            let stone_str = stone.to_string();
            if stone == 0 {
                stones[i] = 1 as u64;
            } else if stone_str.len() % 2 == 0 {
                let (first, last) = stone_str.split_at(stone.to_string().len() / 2);
                let (first_int, last_int) = (first.parse().unwrap(), last.parse().unwrap());
                // println!("str:{stone_str} first:{first}->{first_int}, last:{last}->{last_int}");
                stones[i] = first_int;
                stones.insert(i + 1, last_int);
                i += 1;
            } else {
                // println!("times {stone}");
                stones[i] *= 2024;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day11::part1::{blink, run};

    #[test]
    pub fn test_blink() {
        let tests = vec![
            (vec![125, 17], vec![253000, 1, 7]),
            (vec![253000, 1, 7], vec![253, 0, 2024, 14168]),
        ];
        for (mut input, expected) in tests {
            blink(&mut input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    pub fn test_file_part1() {
        let result = run("./day11.test.txt");
        assert_eq!(result, 55312);
    }
}
