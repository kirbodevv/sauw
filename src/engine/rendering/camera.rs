use bevy::camera::ScalingMode;
use bevy::prelude::*;

use crate::constants::VIEWPORT_WIDTH;
use crate::engine::player::components::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic({
            let mut ortho = OrthographicProjection::default_2d();
            ortho.scaling_mode = ScalingMode::FixedHorizontal {
                viewport_width: VIEWPORT_WIDTH,
            };
            ortho
        }),
        MainCamera,
    ));
}

pub fn camera_follow(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
    camera.translation.z = 100.0;
}
