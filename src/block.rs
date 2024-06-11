pub const TOTAL_SPECIES: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub species: Species,
}
impl Block {
    pub fn new() -> Block {
        Block {
            species: Species::Air
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Species {
    Air = 0,

    Stone = 1,
    Dirt = 2,
}
