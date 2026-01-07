use crate::game::world::components::ChunkCoord;
use bevy::ecs::resource::Resource;
use std::collections::HashSet;

#[derive(Resource)]
pub struct WorldSeed(pub u32);

#[derive(Resource)]
pub struct LoadedChunks {
    pub set: HashSet<ChunkCoord>,
}

#[derive(Resource)]
pub struct Settings {
    pub load_radius: i32,
}

impl LoadedChunks {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}
