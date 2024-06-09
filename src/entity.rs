pub trait Entity {
    const ENTITY_TYPE: EntityType;

    fn new() -> Self;

    // fn movement();

    // fn damage(&mut self);
}

pub enum EntityType {
    Player
}
