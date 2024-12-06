use array2d::Array2D;
use guard::Guard;
use map_tile::MapTile;
use utils::{read_lines, time};

mod guard;
mod map_tile;

fn main() {
    let (map, guard) = build_map("src/map.txt");
    let visited_tiles = time(|| walk_map(&mut map.clone(), &mut guard.clone()));
    println!("Part 1: The Guard visited {} tiles (took {} seconds)", visited_tiles.0, visited_tiles.1.as_secs_f32());
    let blocked_tiles = time(|| block_map(&mut map.clone(), &mut guard.clone()));
    println!("Part 2: There are {} possible block positions (took {} seconds)", blocked_tiles.0, blocked_tiles.1.as_secs_f32());
}

fn build_map(filename: &str) -> (Array2D<MapTile>, Guard) {
    let lines = read_lines(&filename);
    let guard_row = lines.iter().position(|l| l.contains("^")).unwrap();
    let guard_col = lines[guard_row].find("^").unwrap();
    let raw_tiles: Vec<Vec<MapTile>> = lines
        .iter()
        .map(|l| l.chars().map(MapTile::new).collect())
        .collect();
    (
        Array2D::from_rows(&raw_tiles).unwrap(),
        Guard::new(guard_row, guard_col),
    )
}

fn walk_map(map: &mut Array2D<MapTile>, guard: &mut Guard) -> usize {
    while !guard.escaped {
        let r = guard.walk(map);
        if r.is_ok_and(|t| t) {
            break;
        }
    }
    map.elements_row_major_iter().filter(|t| t.visited).count()
}

fn block_map(map: &Array2D<MapTile>, guard: &Guard) -> usize {
    // Walk a copy of the map from the starting point so that we can use it
    // to filter which tiles should be tried
    let mut walked_map = map.clone();
    let mut walked_guard = guard.clone();
    let _ = walk_map(&mut walked_map, &mut walked_guard);

    // Iterate through the whole map and check if blocking a tile would cause a loop
    let mut block_possibilities: usize = 0;
    for row in 0..map.row_len() {
        for col in 0..map.column_len() {
            if !walked_map.get(row, col).unwrap().visited {
                continue;
            }
            let mut new_map = map.clone();
            let mut new_guard = guard.clone();
            let test_tile = new_map.get_mut(row, col).unwrap();
            if test_tile.obstacle {
                continue;
            } else {
                test_tile.obstacle = true;
            }
            while !guard.escaped && !guard.stuck {
                let r = new_guard.walk(&mut new_map);
                if r.is_err() {
                    // got stuck, increment possibility counter
                    block_possibilities += 1;
                    break;
                } else if r.is_ok_and(|t| t) {
                    break;
                }
            }
        }
    }
    block_possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (mut map, mut guard) = build_map("src/test-map.txt");
        let visited_tiles = walk_map(&mut map, &mut guard);
        assert_eq!(visited_tiles, 41);
    }

    #[test]
    fn test_part_2() {
        let (map, guard) = build_map("src/test-map.txt");
        let blocked_tiles = block_map(&map, &guard);
        assert_eq!(blocked_tiles, 6);
    }
}
