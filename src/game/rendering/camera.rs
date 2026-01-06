use bevy::camera::ScalingMode;
use bevy::prelude::*;

use crate::constants::VIEWPORT_WIDTH;
use crate::game::player::components::Player;

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
    time: Res<Time>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    let target = Vec3::new(player.translation.x, player.translation.y, 100.0);
    let speed = 5.0;
    camera.translation = camera.translation.lerp(target, speed * time.delta_secs());
}
