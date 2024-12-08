pub mod part1 {

    use std::{collections::HashMap, fs};

    pub fn run(file_path: &str) -> i32 {
        let puzzle = fs::read_to_string(file_path).unwrap();
        let (rules, lines) = parse_puzzle(&puzzle);
        println!("got puzzle {:?} {:?}", rules, lines);
        let mut count = 0;

        'outer: for line in &lines {
            let mut prev_updates = HashMap::new();
            for update in line {
                if let Some(rules) = rules.get(&update) {
                    for rule in rules {
                        if let Some(_) = prev_updates.get(&rule) {
                            continue 'outer;
                        }
                    }
                }
                prev_updates.entry(update).or_insert(true);
            }
            count += line[line.len() / 2];
            println!("no rules broken, count now {}", count);
        }
        println!("count is {}", count);
        return count;
    }

    #[derive(Debug)]
    pub struct Rule {
        pub left: i32,
        pub right: i32,
    }

    pub fn parse_puzzle(puzzle: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut updates = Vec::new();
        let mut is_input_section = false;
        for line in puzzle.lines() {
            if !is_input_section {
                if line.is_empty() {
                    is_input_section = true;
                } else {
                    let rule: Vec<i32> =
                        line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
                    rules
                        .entry(rule[0])
                        .and_modify(|rules| rules.push(rule[1]))
                        .or_insert(vec![rule[1]]);
                }
            } else {
                updates.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
            }
        }
        (rules, updates)
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    pub fn test_parse_puzzle() {
        let input = "\
12|34

12,34,56,78";
        let (rules, lines) = part1::parse_puzzle(input);
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
}
