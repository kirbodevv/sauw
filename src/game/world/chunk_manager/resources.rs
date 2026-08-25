use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::game::world::{ChunkCoord, types::ChunkBlocks};

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub entities: HashMap<ChunkCoord, Entity>,
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

#[derive(Default, Resource)]
pub struct PendingChunkSpawns {
    pub set: HashSet<ChunkCoord>,
}
