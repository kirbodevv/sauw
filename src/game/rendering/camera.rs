use bevy::camera::ScalingMode;
use bevy::prelude::*;

use crate::constants::VIEWPORT_WIDTH;
use crate::game::GameState;
use crate::game::player::Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct TargetCameraZoom(pub f32);

pub fn spawn_camera(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
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
    state.set(GameState::SpawnPlayer)
}

pub fn zoom_camera(
    time: Res<Time>,
    camera_query: Single<&mut Projection, With<MainCamera>>,
    target_zoom: Res<TargetCameraZoom>,
) {
    let speed = 3.0;

    match *camera_query.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            orthographic.scale = orthographic
                .scale
                .lerp(target_zoom.0, speed * time.delta_secs());
        }
        _ => (),
    }
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
