use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::rendering::TargetCameraZoom;

#[derive(Parser, ConsoleCommand)]
#[command(name = "camzoom")]
pub struct CamZoomCommand {
    zoom: Option<f32>,
}

pub fn cam_zoom_command(
    mut log: ConsoleCommand<CamZoomCommand>,
    mut target_camera_zoom: ResMut<TargetCameraZoom>,
) {
    if let Some(Ok(CamZoomCommand { zoom })) = log.take() {
        let zoom = zoom.unwrap_or(1.0);
        target_camera_zoom.0 = zoom;
    }
}
