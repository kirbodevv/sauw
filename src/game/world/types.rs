use bevy::prelude::*;

use crate::{
    constants::{CHUNK_LAYER_VOLUME, CHUNK_SIZE, CHUNK_WORLD},
    game::registry::block_registry::BlockId,
};

#[inline]
pub fn idx_2d(x: usize, y: usize) -> usize {
    x + y * CHUNK_SIZE
}

#[derive(Clone, Copy)]
pub struct ChunkBlocks {
    pub ground: [BlockId; CHUNK_LAYER_VOLUME],
    pub objects: [BlockId; CHUNK_LAYER_VOLUME],
}

impl Default for ChunkBlocks {
    fn default() -> Self {
        Self {
            ground: [BlockId::AIR; CHUNK_LAYER_VOLUME],
            objects: [BlockId::AIR; CHUNK_LAYER_VOLUME],
        }
    }
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
    pub fn from_world_block_pos(x: i32, y: i32) -> Self {
        Self::from_world_pos(x as f32, y as f32)
    }

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
