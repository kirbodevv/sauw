use bevy::prelude::*;

use crate::game::world::camera::MainCamera;

#[derive(Resource)]
pub struct TargetCameraZoom(pub f32);

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
