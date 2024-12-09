use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
    pub id: usize,
    pub empty: bool,
    shadow_id: Uuid,
}

impl Block {
    pub fn new(position: usize, size: u32, empty: bool) -> Vec<Block> {
        let mut result: Vec<Block> = Vec::new();
        for _ in 0..size {
            result.push(Block {
                id: position,
                empty: empty,
                shadow_id: Uuid::new_v4(),
            });
        }
        result
    }
}