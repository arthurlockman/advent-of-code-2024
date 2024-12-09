use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct File {
    pub id: usize,
    pub empty: bool,
    pub size: usize,
    shadow_id: Uuid,
}

impl File {
    pub fn new(position: usize, size: u32, empty: bool) -> File {
        File {
            id: position,
            empty: empty,
            size: size as usize,
            shadow_id: Uuid::new_v4(),
        }
    }

    pub fn checksum(&self, start_position: usize) -> usize {
        if self.empty {
            return 0;
        }
        (start_position .. start_position + self.size).map(|p| self.id * p).sum()
    }

    pub fn print(&self) -> String {
        let char: String = match self.empty {
            true => ".".to_string(),
            false => self.id.to_string()
        };
        "[".to_string() + &char.repeat(self.size) + "]"
    }
}