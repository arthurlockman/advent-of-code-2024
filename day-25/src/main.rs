use itertools::Itertools;
use utils::{read_file, time};

fn main() {
    let (part1, time1) = time(|| part_1("src/input.txt"));
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
}

fn part_1(filename: &str) -> usize {
    let (locks, keys) = parse_keys_and_locks(filename);
    let mut successes = 0usize;
    for lock in &locks {
        for key in &keys {
            let mut fail = false;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fail = true;
                    break;
                }
            }
            if !fail {
                successes += 1;
            }
        }
    }
    successes
}

fn parse_keys_and_locks(filename: &str) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let file = read_file(filename);
    let mut keys = Vec::<Vec<i8>>::new();
    let mut locks = Vec::<Vec<i8>>::new();
    for item in file.split("\n\n") {
        let lock = item.starts_with("#");
        let lines = if lock {
            item.lines().into_iter().collect_vec()
        } else {
            item.lines().into_iter().rev().collect_vec()
        };
        let mut r = vec![-1i8; 5];
        for (line_no, line) in lines.into_iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '.' && r[col] == -1 {
                    r[col] = line_no as i8 - 1;
                }
            }
        }
        if lock {
            locks.push(r);
        } else {
            keys.push(r);
        }
    }
    (locks, keys)
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/sample.txt"), 3);
    }
}
