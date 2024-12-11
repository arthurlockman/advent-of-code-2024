use std::collections::HashMap;
use memoize::memoize;
use utils::time;

fn main() {
    let stones = "64554 35 906 6 6960985 5755 975820 0";
    let (part1, time1) = time(|| blink(stones, 25));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| blink(stones, 75));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

fn blink(stones: &str, times: u64) -> u64 {
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    for stone in stones.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        stones_map.entry(stone).and_modify(|x| *x += 1).or_insert(1);
    }

    for _ in 0..times {
        let mut ns: HashMap<u64, u64> = HashMap::new();
        for (&key, &val) in &stones_map {
            let new_stones = bl(key);
            for s in new_stones {
                ns.entry(s).and_modify(|x| *x += val).or_insert(val);
            }
        }
        stones_map = ns;
    }
    stones_map.values().sum()
}

#[memoize]
fn bl(n: u64) -> Vec<u64> {
    if n == 0 {
        // If the stone is engraved with the number 0,
        // it is replaced by a stone engraved with the number 1.
        vec![1]
    } else if n.to_string().len() % 2 == 0 {
        // If the stone is engraved with a number that has an even number of digits,
        // it is replaced by two stones. The left half of the digits are engraved
        // on the new left stone, and the right half of the digits are engraved on
        // the new right stone. (The new numbers don't keep extra leading zeroes:
        // 1000 would become stones 10 and 0.)
        let number_string = n.to_string();
        let (left, right) = number_string.split_at(number_string.len() / 2);
        vec![
            left.parse::<u64>().unwrap(),
            right.parse::<u64>().unwrap()
        ]
    } else {
        // If none of the other rules apply, the stone is replaced by a new stone;
        // the old stone's number multiplied by 2024 is engraved on the new stone.
        vec![n * 2024]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = blink("125 17", 25);
        assert_eq!(result, 55312);
    }
}
