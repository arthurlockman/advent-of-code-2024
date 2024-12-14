#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub grid_size_x: i32,
    pub grid_size_y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, grid_size_x: i32, grid_size_y: i32) -> Point {
        Point { x, y, grid_size_x, grid_size_y }
    }

    pub fn add(&mut self, delta_x: i32, delta_y: i32) {
        self.x += delta_x;
        self.y += delta_y;
        if self.x >= self.grid_size_x {
            self.x -= self.grid_size_x;
        }
        if self.x < 0 {
            self.x += self.grid_size_x;
        }
        if self.y >= self.grid_size_y {
            self.y -= self.grid_size_y;
        }
        if self.y < 0 {
            self.y += self.grid_size_y;
        }
    }
}