#[derive(Debug)]
pub struct Block {
    name: String,
    position: [u64; 2],
    size: [u64; 2],
}

impl Block {
    pub fn new(name: &str, position: [u64; 2], size: [u64; 2]) -> Self {
        Block {
            name: name.to_owned(),
            position,
            size,
        }
    }
}
