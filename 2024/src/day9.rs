pub mod part1 {
    use std::fs;

    #[derive(Debug)]
    pub enum FileBlock {
        FileId(u32),
        Space,
    }

    pub fn run(file_path: &str) -> usize {
        let puzzle = fs::read_to_string(file_path).unwrap().trim().to_string();
        let checksum = get_checksum(puzzle);
        println!("checksum is: {}", checksum);
        checksum
    }

    pub fn get_checksum(puzzle: String) -> usize {
        let mut unzipped = unzip(puzzle);
        compact(&mut unzipped);
        checksum(unzipped)
    }

    pub fn unzip(file_zipped: String) -> Vec<FileBlock> {
        let mut unzipped = Vec::new();

        let mut file_id = 0;
        let mut digits = file_zipped.chars().map(|c| c.to_digit(10).unwrap());
        while let Some(file_size) = digits.next() {
            for _ in 0..file_size {
                unzipped.push(FileBlock::FileId(file_id));
            }
            if let Some(space_size) = digits.next() {
                for _ in 0..space_size {
                    unzipped.push(FileBlock::Space);
                }
            }
            file_id += 1;
        }
        unzipped
    }

    pub fn compact(file_blocks: &mut Vec<FileBlock>) {
        let mut right = file_blocks.len() - 1;
        let mut left = 0;
        while left < right {
            if let (Some(FileBlock::Space), Some(FileBlock::FileId(_))) =
                (file_blocks.get(left), file_blocks.get(right))
            {
                file_blocks.swap(left, right);
                left += 1;
                right -= 1;
            }
            if let Some(FileBlock::FileId(_)) = file_blocks.get(left) {
                left += 1;
            }
            if let Some(FileBlock::Space) = file_blocks.get(right) {
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

pub mod part2 {
    use std::{fmt, fs};

    #[derive(Debug)]
    pub struct File {
        id: u32,
        size: u32,
    }

    #[derive(Debug)]
    pub enum FileBlock {
        File(File),
        Space(u32),
    }

    impl fmt::Display for FileBlock {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FileBlock::File(file) => {
                    write!(f, "{}", file.id.to_string().repeat(file.size as usize))
                }
                FileBlock::Space(space_length) => {
                    write!(f, "{}", ".".repeat(*space_length as usize))
                }
            }
        }
    }

    pub fn run(file_path: &str) -> usize {
        let puzzle = fs::read_to_string(file_path).unwrap().trim().to_string();
        let checksum = get_checksum(puzzle);
        println!("checksum part2 is: {}", checksum);
        checksum
    }

    pub fn get_checksum(puzzle: String) -> usize {
        let (mut unzipped, max_file_id) = unzip(puzzle);
        compact(&mut unzipped, max_file_id);
        checksum(unzipped)
    }

    pub fn unzip(file_zipped: String) -> (Vec<FileBlock>, u32) {
        let mut unzipped = Vec::new();

        let mut id = 0;
        let mut digits = file_zipped.chars().map(|c| c.to_digit(10).unwrap());
        while let Some(size) = digits.next() {
            unzipped.push(FileBlock::File(File { id, size }));
            if let Some(space_size) = digits.next() {
                unzipped.push(FileBlock::Space(space_size));
            }
            id += 1;
        }
        (unzipped, id - 1)
    }

    pub fn find_space(
        file_blocks: &Vec<FileBlock>,
        size: u32,
        max_index: usize,
    ) -> Option<(usize, u32)> {
        file_blocks
            .iter()
            .take(max_index)
            .enumerate()
            .find_map(|(i, block)| match block {
                FileBlock::Space(space_size) if *space_size >= size => Some((i, *space_size)),
                _ => None,
            })
    }

    pub fn compact(file_blocks: &mut Vec<FileBlock>, max_file_id: u32) {
        let mut expected_file_id = max_file_id;
        for i in (0..file_blocks.len()).rev() {
            if let Some(FileBlock::File(file)) = file_blocks.get(i) {
                if expected_file_id != file.id {
                    continue;
                }
                if let Some((j, space_size)) = find_space(file_blocks, file.size, i) {
                    if space_size == file.size {
                        file_blocks.swap(i, j);
                    } else if space_size > file.size {
                        let new_space = FileBlock::Space(space_size - file.size);
                        file_blocks[j] = FileBlock::Space(file.size);
                        file_blocks.swap(i, j);
                        file_blocks.insert(j + 1, new_space);
                    }
                }
                if expected_file_id > 0 {
                    expected_file_id -= 1;
                }
            }
        }
    }

    pub fn checksum(compacted: Vec<FileBlock>) -> usize {
        let mut last_start = 0;
        let mut total = 0;
        for i in 0..compacted.len() {
            let block = compacted.get(i).unwrap();
            match block {
                FileBlock::File(file) => {
                    for i in last_start..last_start + file.size as usize {
                        total += i * file.id as usize
                    }
                    last_start += file.size as usize;
                }
                FileBlock::Space(space_size) => last_start += *space_size as usize,
            }
        }
        total
    }
}

#[cfg(test)]
pub mod tests {

    use crate::day9::{part1, part2};

    #[test]
    pub fn test_file_run() {
        let checksum = part1::get_checksum("2333133121414131402".to_string());
        assert_eq!(checksum, 1928);
    }

    #[test]
    pub fn test_run() {
        let result = part1::run("./day9.txt");
        assert_eq!(result, 6258319840548);
    }

    #[test]
    pub fn test_file_run_2() {
        let checksum = part2::get_checksum("2333133121414131402".to_string());
        assert_eq!(checksum, 2858);
    }

    #[test]
    pub fn test_run_2() {
        let result = part2::run("./day9.txt");
        assert_eq!(result, 6286182965311);
    }
}
