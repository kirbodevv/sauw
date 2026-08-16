use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}

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
        app.insert_resource(WorldConfig::default())
            .add_systems(Startup, configure_physics);
    }
}
