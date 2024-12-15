use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ObjectType {
    Wall,
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
                _ => panic!(),
            },
        }
    }

    pub fn try_push(
        &self,
        direction: Direction,
        position: (i32, i32),
        map: HashMap<(i32, i32), Object>,
    ) -> (HashMap<(i32, i32), Object>, bool) {
        if self.kind == ObjectType::Wall {
            (map, false)
        } else {
            // Directions are (row, col)
            let mov = match direction {
                Direction::North => (-1, 0),
                Direction::South => (1, 0),
                Direction::East => (0, 1),
                Direction::West => (0, -1),
            };
            let row = position.0 + mov.0;
            let col = position.1 + mov.1;
            if let Some(next) = map.get(&(row, col)) {
                if next.kind == ObjectType::Wall {
                    // Next square is a wall, no moving here
                    (map, false)
                } else {
                    // Next square is another box. Need to see if we can push the next box too
                    let (mut new_map, push_result) =
                        next.try_push(direction, (row, col), map.clone());
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