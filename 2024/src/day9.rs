pub mod part1 {
    use core::fmt;
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

pub mod part2 {
    use std::{fmt, fs};

    pub fn run(file_path: &str) -> usize {
        let disk = fs::read_to_string(file_path).unwrap();
        let checksum = get_checksum(disk);
        println!("checksum part2 is: {}", checksum);
        checksum
    }

    pub fn get_checksum(puzzle: String) -> usize {
        let (mut unzipped, max_file_id) = unzip(puzzle);
        compact(&mut unzipped, max_file_id);
        checksum(unzipped)
    }

    #[derive(Debug)]
    pub struct File {
        id: i32,
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

    pub fn unzip(disk: String) -> (Vec<FileBlock>, i32) {
        let mut unzipped = Vec::new();

        let mut file_id = 0;
        let mut chars = disk.chars();
        while let Some(char) = chars.next() {
            if let Some(size) = char.to_digit(10) {
                unzipped.push(FileBlock::File(File { id: file_id, size }));
            }
            if let Some(blank) = chars.next().unwrap_or('0').to_digit(10) {
                unzipped.push(FileBlock::Space(blank));
            }
            file_id += 1;
        }

        (unzipped, file_id - 1)
    }

    pub fn compact(file_blocks: &mut Vec<FileBlock>, max_file_id: i32) {
        let mut expected_file_id = max_file_id;
        for i in (0..file_blocks.len()).rev() {
            if let Some(FileBlock::File(file)) = file_blocks.get(i) {
                if file.id != expected_file_id {
                    continue;
                }
                // for i in 0..file_blocks.len() {
                //     print!("{}", file_blocks[i]);
                // }
                // println!("\ni:{}", i);
                if let Some((j, FileBlock::Space(space_size))) = file_blocks
                    .iter()
                    .take(i)
                    .enumerate()
                    .find(|(_, block)| match block {
                        FileBlock::Space(space_size) if *space_size >= file.size => true,
                        _ => false,
                    })
                {
                    // println!(
                    //     "found size:{} at j: {} for file_id:{}",
                    //     *space_size, j, file.id
                    // );
                    if *space_size == file.size {
                        file_blocks.swap(i, j);
                    } else if *space_size > file.size {
                        let new_space = FileBlock::Space(space_size - file.size);
                        file_blocks[j] = FileBlock::Space(file.size);
                        file_blocks.swap(i, j);
                        file_blocks.insert(j + 1, new_space);
                    }
                }
                expected_file_id -= 1;
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
}
