use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_move(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

pub trait AddMove {
    fn add(self, other: Direction) -> (i32, i32);
}

impl AddMove for (i32, i32) {
    fn add(self, other: Direction) -> (i32, i32) {
        let (x, y) = other.to_move();
        (self.0 + x, self.1 + y)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ObjectType {
    Wall,
    BoxLeft,
    BoxRight,
    Box,
}

#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub(crate) kind: ObjectType,
}

impl Object {
    pub fn new(kind: char) -> Object {
        Object {
            kind: match kind {
                '#' => ObjectType::Wall,
                'O' => ObjectType::Box,
                '[' => ObjectType::BoxLeft,
                ']' => ObjectType::BoxRight,
                _ => panic!(),
            },
        }
    }

    pub fn try_push(
        &self,
        direction: Direction,
        position: (i32, i32),
        map: HashMap<(i32, i32), Object>,
        from_pair: bool
    ) -> (HashMap<(i32, i32), Object>, bool) {
        let mut map = map.clone();
        if self.kind == ObjectType::Wall {
            (map, false)
        } else {
            if !from_pair && (self.kind == ObjectType::BoxLeft || self.kind == ObjectType::BoxRight) &&
                (direction == Direction::North || direction == Direction::South) {
                // We're a left or right box so we have to move in tandem with our pair.
                // This means we just have to only move if our pair can move too.
                // `from_pair` indicates that we're coming from a pair so we shouldn't
                // add the one we came from to the check. We only care about N/S moves here
                // since E/W will be handled correctly by the normal logic
                let p = match self.kind {
                    ObjectType::BoxLeft => position.add(Direction::East),
                    ObjectType::BoxRight => position.add(Direction::West),
                    _ => unreachable!(),
                };
                if let Some(pair) = map.get(&p) {
                    let (nm, push_result) = pair.try_push(direction, p, map.clone(), true);
                    if !push_result {
                        // Pair wasn't able to move, so the map doesn't change and we don't move
                        return (map, false);
                    }
                    map = nm;
                    // Pair was able to move, continue as normal
                }
            }
            // Directions are (row, col)
            let (row, col) = position.add(direction);
            if let Some(next) = map.get(&(row, col)) {
                if next.kind == ObjectType::Wall {
                    // Next square is a wall, no moving here
                    (map, false)
                } else {
                    // Next square is another box. Need to see if we can push the next box too
                    let (mut new_map, push_result) =
                        next.try_push(direction, (row, col), map.clone(), false);
                    if push_result {
                        // The next square over moved! We can move too.
                        let o = new_map.remove(&(position.0, position.1)).unwrap();
                        new_map.insert((row, col), o);
                        (new_map, true)
                    } else {
                        // Next square didn't move, so we didn't move either
                        (map, false)
                    }
                }
            } else {
                // Next square is empty, we can move!
                let mut new_map = map.clone();
                let o = new_map.remove(&(position.0, position.1)).unwrap();
                new_map.insert((row, col), o);
                (new_map, true)
            }
        }
    }
}
