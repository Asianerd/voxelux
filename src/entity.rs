use bevy::prelude::*;

pub trait Entity {
    const ENTITY_TYPE: EntityType;
    const SPECIES: Species;

    fn new() -> Self;

    // fn movement();
}

pub enum EntityType {
    Player,

    Enemy,
}

pub enum Species {
    Player,

    Zombie,
    Skeleton,
}

#[derive(Component)]
pub struct Feet;
