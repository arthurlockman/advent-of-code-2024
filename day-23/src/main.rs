use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;
use utils::{read_lines, time};

fn main() {
    let (part1, time1) = time(|| part_1("src/input.txt"));
    println!(
        "Part 1: {} (took {} milliseconds)",
        part1,
        time1.as_millis()
    );
    let (part2, time2) = time(|| part_2("src/input.txt"));
    println!("Part 2: {} (took {} μs)", part2, time2.as_micros());
}

fn part_1(filename: &str) -> usize {
    let ed = load_network(filename);
    let edges = ed
        .iter()
        .map(|s| (s.0.as_str(), s.1.as_str()))
        .collect_vec();
    let network = UnGraphMap::<_, ()>::from_edges(edges);
    let mut groups: HashSet<Vec<String>> = HashSet::new();
    for n1 in network.nodes() {
        for n2 in network.neighbors(n1) {
            for n3 in network.neighbors(n2) {
                for n4 in network.neighbors(n3) {
                    if n4 == n1 {
                        // Interconnected node found
                        groups.insert(
                            vec![n1.to_string(), n2.to_string(), n3.to_string()]
                                .into_iter()
                                .sorted()
                                .collect_vec(),
                        );
                    }
                }
            }
        }
    }
    groups
        .into_iter()
        .filter(|n| n.iter().any(|x| x.starts_with("t")))
        .count()
}

fn part_2(filename: &str) -> String {
    let ed = load_network(filename);
    let edges = ed
        .iter()
        .map(|s| (s.0.as_str(), s.1.as_str()))
        .collect_vec();
    let network = UnGraphMap::<_, ()>::from_edges(edges);
    let mut largest = vec![];
    for node in network.nodes() {
        let mut g = vec![node];
        let neighbors = network.neighbors(node);
        for neighbor in neighbors {
            if g.iter().all(|n| network.neighbors(neighbor).contains(n)) {
                g.push(neighbor);
            }
        }
        g.sort();
        if g.len() > largest.len() {
            largest = g;
        }
    }
    largest.iter().join(",")
}

fn load_network(filename: &str) -> Vec<(String, String)> {
    read_lines(filename)
        .iter()
        .map(|l| {
            let split = l.split('-').collect_vec();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/sample.txt"), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/sample.txt"), "co,de,ka,ta");
    }
}
