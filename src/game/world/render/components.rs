use bevy::prelude::*;

use crate::game::world::ChunkCoord;

#[derive(Component)]
pub struct BlockEntity;

#[derive(Component)]
pub struct ChunkMesh {
    pub coord: ChunkCoord,
}

#[derive(Component, Clone)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub layer: u8,
}

impl BlockPos {
    pub fn new(x: u8, y: u8, layer: u8) -> Self {
        Self { x, y, layer }
    }
}
