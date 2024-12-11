use indicatif::ProgressBar;
use utils::time;
use rayon::prelude::*;

struct Stone {
    number: u64,
}

impl Stone {
    fn new(number: u64) -> Stone {
        Stone { number }
    }

    fn blink(&self) -> Vec<Stone> {
        if self.number == 0 {
            // If the stone is engraved with the number 0,
            // it is replaced by a stone engraved with the number 1.
            vec![Stone::new(1)]
        } else if self.number.to_string().len() % 2 == 0 {
            // If the stone is engraved with a number that has an even number of digits,
            // it is replaced by two stones. The left half of the digits are engraved
            // on the new left stone, and the right half of the digits are engraved on
            // the new right stone. (The new numbers don't keep extra leading zeroes:
            // 1000 would become stones 10 and 0.)
            let number_string = self.number.to_string();
            let (left, right) = number_string.split_at(number_string.len() / 2);
            vec![
                Stone::new(left.parse::<u64>().unwrap()),
                Stone::new(right.parse::<u64>().unwrap()),
            ]
        } else {
            // If none of the other rules apply, the stone is replaced by a new stone;
            // the old stone's number multiplied by 2024 is engraved on the new stone.
            vec![Stone::new(self.number * 2024)]
        }
    }
}

fn main() {
    let stones = "64554 35 906 6 6960985 5755 975820 0";
    let (part1, time1) = time(|| blink(stones, 25));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| blink(stones, 75));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

fn blink(stones: &str, times: usize) -> usize {
    let mut stones: Vec<Stone> = stones
        .split_whitespace()
        .map(|s| Stone::new(s.parse::<u64>().unwrap()))
        .collect();
    for _ in 0..times {
        stones = stones.par_iter().flat_map(|stone| stone.blink()).collect();
    }
    stones.len()
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
