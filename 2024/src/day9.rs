pub mod part1 {
    use std::fs;

    pub fn run(file_path: &str) -> usize {
        let disk = fs::read_to_string(file_path).unwrap();
        let checksum = get_checksum(disk);
        println!("checksum is: {}", checksum);
        checksum
    }

    pub fn get_checksum(puzzle: String) -> usize {
        let mut unzipped = unzip(puzzle);
        compact(&mut unzipped);
        checksum(unzipped)
    }

    #[derive(Debug)]
    pub enum FileBlock {
        FileId(i32),
        Space,
    }

    pub fn unzip(disk: String) -> Vec<FileBlock> {
        let mut unzipped = Vec::new();

        let mut file_id = 0;
        let mut chars = disk.chars();
        while let Some(char) = chars.next() {
            if let Some(size) = char.to_digit(10) {
                for _ in 0..size {
                    unzipped.push(FileBlock::FileId(file_id));
                }
            }
            if let Some(blank) = chars.next().unwrap_or('0').to_digit(10) {
                for _ in 0..blank {
                    unzipped.push(FileBlock::Space);
                }
            }
            file_id += 1;
        }
        unzipped
    }

    pub fn compact(unzipped: &mut Vec<FileBlock>) {
        let mut right = unzipped.len() - 1;
        let mut left = 0;
        while left < right {
            if let (Some(FileBlock::Space), Some(FileBlock::FileId(_))) =
                (unzipped.get(left), unzipped.get(right))
            {
                unzipped.swap(left, right);
                left += 1;
                right -= 1;
            }
            if let Some(FileBlock::FileId(_)) = unzipped.get(left) {
                left += 1;
            }
            if let Some(FileBlock::Space) = unzipped.get(right) {
                right -= 1;
            }
        }
    }

    pub fn checksum(compacted: Vec<FileBlock>) -> usize {
        compacted
            .iter()
            .enumerate()
            .filter_map(|(i, file_block)| match file_block {
                FileBlock::FileId(file_id) => Some(*file_id as usize * i),
                _ => None,
            })
            .sum()
    }
}

#[cfg(test)]
pub mod tests {

    use crate::day9::part1::{get_checksum, run};

    #[test]
    pub fn test_file_run() {
        let checksum = get_checksum("2333133121414131402".to_string());
        assert_eq!(checksum, 1928);
    }

    #[test]
    pub fn test_run() {
        let result = run("./day9.txt");
        assert_eq!(result, 6258319840548);
    }
}
