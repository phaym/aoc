use std::fs;

pub fn run() {
    println!("test");
    let puzzle = fs::read_to_string("./day5.test.txt").unwrap();
    let (rules, lines) = parse_puzzle(&puzzle);
    println!("got puzzle {:?} {:?}", rules, lines);
}

#[derive(Debug)]
pub struct Rule {
    pub left: i32,
    pub right: i32,
}

pub fn parse_puzzle(puzzle: &str) -> (Vec<Rule>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut input = Vec::new();
    let mut is_input_section = false;
    puzzle.lines().into_iter().for_each(|line| {
        if !is_input_section {
            if line.is_empty() {
                is_input_section = true;
            } else {
                let rule: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
                rules.push(Rule {
                    left: rule[0],
                    right: rule[1],
                });
            }
        } else {
            input.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        }
    });
    (rules, input)
}

#[cfg(test)]
pub mod tests {
    use crate::day5::parse_puzzle;

    #[test]
    pub fn test_parse_puzzle() {
        let input = "\
12|34

12,34,56,78";
        let (rules, lines) = parse_puzzle(input);
        assert_eq!(rules[0].left, 12);
        assert_eq!(rules[0].right, 34);
        assert_eq!(lines, vec![vec![12, 34, 56, 78]]);
    }
}
