use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_firefly::data::{FireflyConfig, NormalMode};

use crate::constants::VIEWPORT_WIDTH;
use crate::game::player::Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct TargetCameraZoom(pub f32);

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
        FireflyConfig {
            normal_mode: NormalMode::TopDownY,
            enable_32bit_stencils: true,
            ..default()
        },
        Msaa::Off,
    ));
}

pub fn zoom_camera(
    time: Res<Time>,
    camera_query: Single<&mut Projection, With<MainCamera>>,
    target_zoom: Res<TargetCameraZoom>,
) {
    let speed = 3.0;

    if let Projection::Orthographic(ref mut orthographic) = *camera_query.into_inner() {
        orthographic.scale = orthographic
            .scale
            .lerp(target_zoom.0, speed * time.delta_secs());
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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetCameraZoom(1.0))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (zoom_camera, camera_follow));
    }
}
