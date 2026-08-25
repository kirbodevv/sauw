use bevy::prelude::*;

pub mod follow;
pub mod spawner;
pub mod zoom;

pub use zoom::TargetCameraZoom;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetCameraZoom(1.0))
            .add_systems(Startup, spawner::spawn_camera)
            .add_systems(Update, (zoom::zoom_camera, follow::camera_follow));
    }
}
