use bevy::prelude::*;

#[derive(Clone)]
pub struct WorldSeed(pub u32);

#[derive(Resource)]
pub struct WorldConfig {
    pub seed: WorldSeed,
    pub load_radius: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: WorldSeed(0),
            load_radius: 2,
        }
    }
}

pub struct WorldConfigPlugin;

impl Plugin for WorldConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default());
    }
}
