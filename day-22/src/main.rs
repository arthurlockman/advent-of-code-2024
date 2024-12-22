use itertools::{iterate, Itertools};
use rayon::prelude::*;
use std::collections::HashMap;
use utils::{read_lines, time};

fn main() {
    let (part1, time1) = time(|| part_1());
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| part_2());
    println!("Part 2: {} (took {} secs)", part2, time2.as_secs_f64());
}

fn part_1() -> i64 {
    read_lines("src/input.txt")
        .par_iter()
        .map(|l| l.parse::<i64>())
        .map(|n| nth_secret(n.unwrap(), 2000))
        .sum()
}

fn step(mut secret_num: i64) -> i64 {
    // Step 1
    secret_num = (secret_num * 64) ^ secret_num;
    secret_num = secret_num % 16777216;
    // Step 2
    secret_num = (secret_num / 32) ^ secret_num;
    secret_num = secret_num % 16777216;
    // Step 3
    secret_num = (secret_num * 2048) ^ secret_num;
    secret_num % 16777216
}

fn part_2() -> i64 {
    // Reminds me of LINQ......
    read_lines("src/input.txt")
        .par_iter()
        .map(|l| l.parse::<i64>())
        .map(|p| {
            let prices = iterate(p.unwrap(), |secret_num: &i64| step(*secret_num))
                .take(2001)
                .map(|n| n % 10)
                .collect_vec();
            prices
                .into_iter()
                .rev()
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((d - e, c - d, b - c, a - b), a))
                .collect::<HashMap<_, _>>()
        })
        .reduce(
            || HashMap::new(),
            |mut accumulator, val| {
                val.into_iter()
                    .for_each(|(k, v)| *accumulator.entry(k).or_insert(0) += v);
                accumulator
            },
        )
        .into_values()
        .max()
        .unwrap()
}

fn nth_secret(mut secret_num: i64, n: usize) -> i64 {
    for _ in 0..n {
        secret_num = step(secret_num);
    }
    secret_num
}

#[cfg(test)]
mod tests {
    use crate::nth_secret;

    #[test]
    fn test_part_1() {
        assert_eq!(nth_secret(1, 2000), 8685429);
        assert_eq!(nth_secret(10, 2000), 4700978);
        assert_eq!(nth_secret(100, 2000), 15273692);
        assert_eq!(nth_secret(2024, 2000), 8667524);
    }
}
