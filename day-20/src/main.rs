use itertools::Itertools;
use pathfinding::num_traits::abs;
use pathfinding::prelude::dijkstra;
use std::iter::repeat;
use utils::{parse_map, time, Tile};

fn main() {
    let (part1, time1) = time(|| cheat("src/input.txt", 2, 100));
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| cheat("src/input.txt", 20, 100));
    println!("Part 2: {} (took {} secs)", part2, time2.as_secs_f64());
}

fn cheat(filename: &str, cheat_duration: i32, threshold: i32) -> i32 {
    let grid = parse_map(filename, |c: char| match c {
        '#' => Tile::Wall,
        '.' => Tile::Path,
        'S' => Tile::Start,
        'E' => Tile::Exit,
        _ => panic!("Unexpected char {}", c),
    });
    let (start, _) = grid.iter().find(|(_, v)| **v == Tile::Start).unwrap();
    let (end, _) = grid.iter().find(|(_, v)| **v == Tile::Exit).unwrap();
    let (shortest_path, _) = dijkstra(
        start,
        |p| {
            p.neighbors()
                .into_iter()
                .filter(|(x, _)| *grid.get(x).unwrap() != Tile::Wall)
                .collect_vec()
        },
        |p| p == end,
    )
    .unwrap();
    let possible_cheats = repeat(shortest_path.iter())
        .take(2)
        .multi_cartesian_product()
        .map(|x| (x[0], x[1]))
        .filter(|(x, y)| x != y && abs(x.row - y.row) + abs(x.col - y.col) <= cheat_duration)
        .collect_vec();
    let mut passing_cheats = 0;
    let shortest_path_map = shortest_path.iter().enumerate().into_group_map_by(|x| x.1);
    for (cs, ce) in possible_cheats {
        let (start_pos, _) = shortest_path_map.get(&cs).unwrap()[0];
        let (end_pos, _) = shortest_path_map.get(&ce).unwrap()[0];
        if end_pos as i32 - start_pos as i32 - (abs(cs.row - ce.row) + abs(cs.col - ce.col))
            >= threshold
        {
            passing_cheats += 1;
        }
    }
    passing_cheats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = cheat("src/sample.txt", 2, 1);
        assert_eq!(result, 44);
    }

    #[test]
    fn test_part_2() {
        let result = cheat("src/sample.txt", 20, 50);
        assert_eq!(result, 285);
    }
}
