use bevy::prelude::*;

#[derive(Clone)]
pub struct WorldSeed(pub u32);

#[derive(Resource)]
pub struct WorldConfig {
    pub seed: WorldSeed,
    pub load_radius: i32,
    pub max_chunk_ops_per_frame: usize,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: WorldSeed(0),
            load_radius: 2,
            max_chunk_ops_per_frame: 4,
        }
    }
}

pub struct WorldConfigPlugin;

impl Plugin for WorldConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default());
    }
}
