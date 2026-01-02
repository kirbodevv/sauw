use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::world::chunk::Chunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource)]
pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        self.chunks.entry(pos).or_insert_with(Chunk::new)
    }
}
