use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

#[derive(Parser, ConsoleCommand)]
#[command(name = "camzoom")]
pub struct CamZoomCommand {
    zoom: Option<f32>,
}

pub fn cam_zoom_command(
    mut log: ConsoleCommand<CamZoomCommand>,
    camera_query: Single<&mut Projection, With<Camera>>,
) {
    if let Some(Ok(CamZoomCommand { zoom })) = log.take() {
        let zoom = zoom.unwrap_or(1.0);
        match *camera_query.into_inner() {
            Projection::Orthographic(ref mut orthographic) => {
                orthographic.scale = zoom;
            }
            _ => (),
        }
    }
}
