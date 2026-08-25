use bevy::prelude::*;

use crate::constants::{CHUNK_SIZE, CHUNK_WORLD};

#[inline]
pub fn idx(x: usize, y: usize, layer: usize) -> usize {
    x + y * CHUNK_SIZE + layer * CHUNK_SIZE * CHUNK_SIZE
}

#[inline]
pub fn idx_2d(x: usize, y: usize) -> usize {
    x + y * CHUNK_SIZE
}

#[inline]
pub fn chunk_positions() -> impl Iterator<Item = (usize, usize)> {
    (0..CHUNK_SIZE).flat_map(|x| (0..CHUNK_SIZE).map(move |y| (x, y)))
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

    pub fn to_world_pos(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32) * CHUNK_WORLD
    }
}
