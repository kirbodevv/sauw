use bevy::prelude::*;

use crate::game::registry::item_registry::ItemId;

#[derive(Component)]
pub struct Drop {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Component, Clone, PartialEq, Eq)]
pub enum DroppedBy {
    Player(Entity),
    Spawned,
    Command,
}

#[derive(Component)]
pub struct PickupLocked;

#[derive(Message)]
pub struct SpawnDrop {
    pub id: ItemId,
    pub count: u32,
    pub position: Vec2,
    pub dropped_by: DroppedBy,
}
