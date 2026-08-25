use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::game::world::{ChunkCoord, types::ChunkBlocks};

#[derive(Resource, Default)]
pub struct ChunkManager {
    entities: HashMap<ChunkCoord, Entity>,
}

impl ChunkManager {
    pub fn is_spawned(&self, coord: &ChunkCoord) -> bool {
        self.entities.contains_key(coord)
    }

    pub fn entity(&self, coord: &ChunkCoord) -> Option<Entity> {
        self.entities.get(coord).copied()
    }

    pub fn register(&mut self, coord: ChunkCoord, entity: Entity) {
        self.entities.insert(coord, entity);
    }

    pub fn unregister(&mut self, coord: &ChunkCoord) -> Option<Entity> {
        self.entities.remove(coord)
    }
}

#[derive(Resource, Default)]
pub struct ChunkBlocksStore {
    pub blocks: HashMap<ChunkCoord, ChunkBlocks>,
}

#[derive(Resource, Default)]
pub struct RequiredChunks {
    pub set: HashSet<ChunkCoord>,
    pub ordered: Vec<ChunkCoord>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChunkLoadState {
    Generating,
    AwaitingSpawn,
}

#[derive(Default, Resource)]
pub struct PendingChunkSpawns {
    pub state: HashMap<ChunkCoord, ChunkLoadState>,
}
