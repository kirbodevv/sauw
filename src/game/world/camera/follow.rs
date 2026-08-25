use bevy::prelude::*;

use crate::game::{player::Player, world::camera::MainCamera};

pub fn camera_follow(
    time: Res<Time>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    let target = Vec3::new(player.translation.x, player.translation.y, 100.0);
    let speed = 5.0;
    camera.translation = camera.translation.lerp(target, speed * time.delta_secs());
}
