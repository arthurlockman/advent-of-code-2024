use crate::guard::Direction;

#[derive(Clone)]
#[derive(Debug)]
pub struct MapTile {
    pub obstacle: bool,
    pub visited: bool,
    visited_from: Vec<Direction>
}

 impl MapTile {
    pub fn new(raw_string: char) -> MapTile {
        MapTile {
            obstacle: raw_string == '#',
            visited: false,
            visited_from: Vec::new()
        }
    }

    pub fn visit(&mut self, direction: Direction) {
        self.visited = true;
        self.visited_from.push(direction);
    }

    pub fn has_been_visited(&self, direction: Direction) -> bool {
        self.visited && self.visited_from.contains(&direction)
    }
}