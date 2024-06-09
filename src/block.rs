use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Stone,
    Dirt
}

pub const TOTAL_TYPES: i32 = 3;
// so stupid

pub struct Block {
    pub blocktype: BlockType
}

pub fn type_to_i32(t: BlockType) -> i32 {
    // stupid i know
    match t {
        BlockType::Stone => 1,
        BlockType::Dirt => 2,
        _ => 0
    }
}
