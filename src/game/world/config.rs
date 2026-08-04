use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}
