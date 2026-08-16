use bevy::prelude::*;

use crate::constants::CHUNK_WORLD;

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
