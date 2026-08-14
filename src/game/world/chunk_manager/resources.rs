use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::game::world::ChunkCoord;

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub entities: HashMap<ChunkCoord, Entity>,
}

#[derive(Resource, Default)]
pub struct RequiredChunks {
    pub set: HashSet<ChunkCoord>,
}
