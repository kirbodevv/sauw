use bevy::camera::ScalingMode;
use bevy::prelude::*;

use crate::constants::VIEWPORT_WIDTH;
use crate::game::player::Player;

#[derive(Component)]
pub struct YSort {
    pub z: f32,
}

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
    ));
}

pub fn apply_y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, ysort) in q.iter_mut() {
        tf.translation.z = ysort.z - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetCameraZoom(1.0))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (zoom_camera, camera_follow, apply_y_sort));
    }
}
