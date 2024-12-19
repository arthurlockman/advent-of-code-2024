use itertools::Itertools;
use std::collections::HashMap;
use trie_rs::{Trie, TrieBuilder};
use utils::{read_file, time};

fn main() {
    let (solution, time1) = time(|| solve("src/input.txt"));
    println!(
        "Part 1: {}\nPart 2: {}\nTook {} seconds to solve",
        solution.0,
        solution.1,
        time1.as_secs_f64()
    );
}

fn solve(filename: &str) -> (usize, usize) {
    let (towels_available, towel_goals) = parse_input(filename);
    let mut memo: HashMap<String, (bool, usize)> = HashMap::new();
    let possible_towels = towel_goals
        .iter()
        .map(|t| can_build(&towels_available, t, &mut memo))
        .collect_vec();
    (
        possible_towels.iter().filter(|b| b.0).count(),
        possible_towels.iter().map(|b| b.1).sum(),
    )
}

fn can_build(
    available: &Trie<u8>,
    towel: &String,
    mut memo: &mut HashMap<String, (bool, usize)>,
) -> (bool, usize) {
    if towel.len() == 0 {
        (true, 1)
    } else if let Some(r) = memo.get(towel) {
        *r
    } else {
        let prefixes: Vec<String> = available.common_prefix_search(&towel).collect_vec();
        let r = prefixes
            .iter()
            .map(|p| can_build(available, &towel[p.len()..].to_string(), &mut memo))
            .filter(|b| b.0)
            .collect_vec();
        if r.len() > 0 {
            let result = (true, r.iter().map(|x| x.1).sum());
            memo.insert(towel.to_string(), result);
            result
        } else {
            memo.insert(towel.to_string(), (false, 0));
            (false, 0)
        }
    }
}

fn parse_input(filename: &str) -> (Trie<u8>, Vec<String>) {
    let input = read_file(filename);
    let raw = input.split("\n\n").collect_vec();
    let mut builder = TrieBuilder::new();
    raw[0]
        .split(", ")
        .map(|x| x.to_string())
        .for_each(|x| builder.push(x));
    (
        builder.build(),
        raw[1].split("\n").map(|x| x.to_string()).collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_both() {
        let r = solve("src/sample.txt");
        assert_eq!(r.0, 6);
        assert_eq!(r.1, 16);
    }
}
