mod block;
mod file;

use block::Block;
use file::File;
use indicatif::ProgressBar;
use itertools::Itertools;
use utils::{read_file, time};

fn main() {
    let part1_disk = parse_disk_to_blocks(&read_file("src/input.txt"));
    let (result1, time1) = time(|| defrag_stupidly(&part1_disk));
    println!(
        "Part 1 Checksum: {} (took {} seconds)",
        result1,
        time1.as_secs_f64()
    );

    let part2_disk = parse_disk_to_files(&read_file("src/input.txt"));
    let (result2, time2) = time(|| defrag(&part2_disk));
    println!(
        "Part 2 Checksum: {} (took {} seconds)",
        result2,
        time2.as_secs_f64()
    );
}

fn parse_disk_to_blocks(disk_str: &str) -> Vec<Block> {
    let mut disk: Vec<Block> = Vec::new();
    let mut data_block_idx = 0;
    for (position, c) in disk_str.chars().enumerate() {
        if position % 2 != 0 {
            // is free space
            disk.append(&mut Block::new(0, c.to_digit(10).unwrap(), true))
        } else {
            // is data block
            disk.append(&mut Block::new(
                data_block_idx,
                c.to_digit(10).unwrap(),
                false,
            ));
            data_block_idx += 1;
        }
    }
    disk
}

fn parse_disk_to_files(disk_str: &str) -> Vec<File> {
    let mut disk: Vec<File> = Vec::new();
    let mut data_block_idx = 0;
    for (position, c) in disk_str.chars().enumerate() {
        if position % 2 != 0 {
            // is free space
            disk.push(File::new(0, c.to_digit(10).unwrap(), true))
        } else {
            // is data block
            disk.push(File::new(data_block_idx, c.to_digit(10).unwrap(), false));
            data_block_idx += 1;
        }
    }
    disk
}

fn defrag_stupidly(disk: &Vec<Block>) -> usize {
    let mut new_disk = disk.clone();
    let mut disk_iter = disk.iter();
    let mut front = disk_iter.next();
    let mut front_idx = 0;
    let mut back = disk_iter.next_back();
    let mut back_idx = disk.len() - 1;
    while front.is_some() && back.is_some() {
        while !front.unwrap().empty {
            front = disk_iter.next();
            front_idx += 1;
        }
        // We've found the first empty block, now find the first
        // occupied block on the end
        while back.is_some() && back.unwrap().empty {
            back = disk_iter.next_back();
            back_idx -= 1;
        }
        // Now we have both an empty block from the front and an occupied
        // block on the end. swap them.
        if back.is_some() && front.is_some() {
            new_disk.swap(front_idx, back_idx);
        }
        front = disk_iter.next();
        front_idx += 1;
        back = disk_iter.next_back();
        back_idx -= 1;
    }
    new_disk
        .iter()
        .enumerate()
        .map(|(idx, blk)| if !blk.empty { blk.id * idx } else { 0 })
        .sum()
}

fn defrag(disk: &Vec<File>) -> usize {
    let max_disk: Vec<usize> = disk
        .iter()
        .filter(|&d| !d.empty)
        .sorted_by_key(|&d| d.id)
        .rev()
        .map(|d| d.id)
        .collect();
    let bar = ProgressBar::new(max_disk[0] as u64);
    let mut new_disk = disk.clone();
    for file_id in disk
        .iter()
        .filter(|&d| !d.empty)
        .sorted_by_key(|&d| d.id)
        .rev()
        .map(|d| d.id)
    {
        bar.set_position((max_disk[0] - file_id) as u64);
        // Locate the file in the disk
        let (file_pos, file) = new_disk
            .iter()
            .find_position(|f| f.id == file_id)
            .map(|(pos, file)| (pos, file.clone()))
            .unwrap();
        // Find an appropriate empty space in the disk that's to the left of the file
        if let Some((empty_pos, empty_block)) = new_disk[0..file_pos]
            .iter()
            .find_position(|f| f.empty && f.size >= file.size)
            .map(|(pos, file)| (pos, file.clone()))
        {
            // We have an appropriate empty block
            if empty_block.size == file.size {
                // Block and file are equal sized, we can swap them
                new_disk.swap(file_pos, empty_pos);
            } else {
                // Block is larger than file. Compact the block, insert
                // the file, and then swap
                new_disk.swap(file_pos, empty_pos);
                let new_empty = File::new(0, (empty_block.size - file.size) as u32, true);
                new_disk[file_pos].size = file.size;
                new_disk.insert(empty_pos + 1, new_empty);
            }
        }
        compact_free_space(&mut new_disk);
    }
    let mut checksum = 0;
    let mut start_pos = 0;
    for file in new_disk {
        checksum += file.checksum(start_pos);
        start_pos += file.size;
    }
    bar.finish_and_clear();
    checksum
}

fn compact_free_space(disk: &mut Vec<File>) {
    let mut new_disk: Vec<File> = Vec::new();
    let mut disk_iter = disk.iter();
    let mut current_file = disk_iter.next();
    while current_file.is_some() {
        let f = current_file.unwrap();
        if !f.empty {
            new_disk.push(f.clone());
            current_file = disk_iter.next();
        } else {
            let mut new_empty = f.clone();
            current_file = disk_iter.next();
            while current_file.is_some() && current_file.unwrap().empty {
                new_empty.size += current_file.unwrap().size;
                current_file = disk_iter.next();
            }
            new_disk.push(new_empty);
        }
    }
    *disk = new_disk;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_disk = "2333133121414131402";
        let disk = parse_disk_to_blocks(example_disk);
        let defrag_result = defrag_stupidly(&disk);
        assert_eq!(defrag_result, 1928);
    }

    #[test]
    fn test_part_2() {
        let example_disk = "2333133121414131402";
        let disk = parse_disk_to_files(example_disk);
        let defrag_result = defrag(&disk);
        assert_eq!(defrag_result, 2858);
    }
}
