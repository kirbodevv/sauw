use crate::engine::world::components::ChunkCoord;
use bevy::ecs::resource::Resource;
use std::collections::HashSet;

#[derive(Resource)]
pub struct LoadedChunks {
    pub set: HashSet<ChunkCoord>,
}

impl LoadedChunks {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}
