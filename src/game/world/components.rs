use bevy::prelude::*;
use std::collections::HashSet;

use crate::constants::CHUNK_WORLD;

#[derive(Component)]
pub struct BlockEntity;

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

#[derive(Component)]
pub struct Chunk;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn from_world_pos(x: f32, y: f32) -> Self {
        Self {
            x: (x / CHUNK_WORLD).floor() as i32,
            y: (y / CHUNK_WORLD).floor() as i32,
        }
    }
}

#[derive(Default, Resource)]
pub struct LoadedChunks {
    pub set: HashSet<ChunkCoord>,
}
