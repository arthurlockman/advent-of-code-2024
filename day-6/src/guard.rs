use array2d::Array2D;

use crate::map_tile::MapTile;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub direction: Direction,
    pub row: usize,
    pub column: usize,
    pub escaped: bool,
    pub stuck: bool,
}

impl Guard {
    pub fn new(row: usize, column: usize) -> Guard {
        Guard {
            direction: Direction::North,
            row: row,
            column: column,
            escaped: false,
            stuck: false,
        }
    }

    /// Walk implements these two basic rules:
    /// - If there is something directly in front of you, turn right 90 degrees.
    /// - Otherwise, take a step forward.
    /// When the guard leaves the map, the "escaped" flag will be set and further
    /// calls to `walk()` will have no effect.
    ///
    /// Returns `Ok(true)` when we've escaped the map, and `Err(true)` when we're stuck in a loop.
    pub fn walk(&mut self, map: &mut Array2D<MapTile>) -> Result<bool, bool> {
        if self.escaped {
            return Ok(true);
        }
        let current_tile = map.get_mut(self.row, self.column);
        current_tile.unwrap().visit(self.direction.clone());

        let mut next_row = self.row as i32;
        let mut next_col = self.column as i32;
        if self.direction == Direction::North {
            next_row -= 1;
        } else if self.direction == Direction::East {
            next_col += 1;
        } else if self.direction == Direction::South {
            next_row += 1;
        } else if self.direction == Direction::West {
            next_col -= 1;
        } else {
            panic!("Unknown map direction: {:?}", self.direction);
        }

        // we've left the map!
        if next_col < 0 || next_row < 0 {
            self.escaped = true;
            return Ok(true);
        }

        let next_tile = map.get(next_row as usize, next_col as usize);

        // we've left the map!
        if next_tile.is_none() {
            self.escaped = true;
            return Ok(true);
        }

        let t = next_tile.unwrap();
        // Uh oh! We've already been here from the same direction. We're stuck.
        if t.has_been_visited(self.direction.clone()) {
            self.stuck = true;
            return Err(true);
        }
        // If we hit an obstacle by moving forward one step, rotate 90º CW and exit.
        if t.obstacle {
            if self.direction == Direction::North {
                self.direction = Direction::East;
            } else if self.direction == Direction::East {
                self.direction = Direction::South;
            } else if self.direction == Direction::South {
                self.direction = Direction::West;
            } else if self.direction == Direction::West {
                self.direction = Direction::North;
            }
        } else {
            // If we haven't hit an obstacle, move forward one tile.
            self.column = next_col as usize;
            self.row = next_row as usize;
        }
        return Ok(false);
    }
}
