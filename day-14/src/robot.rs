use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Robot {
    position: Point,
    velocity_x: i32,
    velocity_y: i32,
}

impl Robot {
    pub fn new(
        position_x: i32,
        position_y: i32,
        velocity_x: i32,
        velocity_y: i32,
        grid_size_x: i32,
        grid_size_y: i32,
    ) -> Robot {
        Robot {
            position: Point::new(position_x, position_y, grid_size_x, grid_size_y),
            velocity_x,
            velocity_y,
        }
    }

    pub fn tick(&mut self) {
        self.position.add(self.velocity_x, self.velocity_y);
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    /// Returns quadrant of the map clockwise from the top left
    pub fn quadrant(&self) -> Option<i32> {
        if self.position.x < self.position.grid_size_x / 2 && self.position.y < self.position.grid_size_y / 2 {
            return Some(1);
        }
        if self.position.x > self.position.grid_size_x / 2 && self.position.y < self.position.grid_size_y / 2 {
            return Some(2);
        }
        if self.position.x > self.position.grid_size_x / 2 && self.position.y > self.position.grid_size_y / 2 {
            return Some(3);
        }
        if self.position.x < self.position.grid_size_x / 2 && self.position.y > self.position.grid_size_y / 2 {
            return Some(4);
        }
        None
    }
}
