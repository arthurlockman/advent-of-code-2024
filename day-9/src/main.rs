mod block;
mod file;
use std::io::empty;

use block::Block;
use file::File;
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
    let mut new_disk = disk.clone();
    let data_blocks_with_positions: Vec<(usize, &File)> = disk
        .iter()
        .enumerate()
        .filter(|&b| !b.1.empty)
        .sorted_by_key(|f| f.1.id)
        .collect();
    for (position, data_block) in data_blocks_with_positions.iter().rev() {
        println!("Block {}", data_block.id);
        if let Some((empty_pos, empty_block)) = new_disk
            .iter()
            .find_position(|&b| b.empty && b.size >= data_block.size)
        {
            if empty_pos <= *position {
                let mut removal_index =
                    new_disk.iter().position(|b| b.id == data_block.id).unwrap();
                // We found empty space before the current block that's big enough
                if empty_block.size == data_block.size {
                    // the space is the same size as the block, so we can just insert the block
                    // where the space was

                    new_disk.swap(empty_pos, removal_index);
                } else {
                    // the space is bigger than the block but we can fix that.
                    // shrink the space and insert the block at the beginning.
                    let mut new_empty_block = empty_block.clone();
                    removal_index += 1;
                    new_empty_block.size = new_empty_block.size - data_block.size;
                    new_disk[empty_pos] = new_empty_block;
                    new_disk.insert(empty_pos, (*data_block).clone());
                    new_disk.remove(removal_index);
                    new_disk.insert(
                        removal_index,
                        File::new(removal_index, data_block.size as u32, true),
                    );
                }
                // finally, compact free space
                compact_free_space(&mut new_disk);
            }
        }
    }
    let mut checksum = 0;
    let mut start_pos = 0;
    for file in new_disk {
        checksum += file.checksum(start_pos);
        start_pos += file.size;
    }
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
