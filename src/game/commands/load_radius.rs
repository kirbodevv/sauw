use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::world::Settings;

#[derive(Parser, ConsoleCommand)]
#[command(name = "loadradius")]
pub struct LoadRadiusCommand {
    radius: i32,
}

pub fn load_radius(
    mut log: ConsoleCommand<LoadRadiusCommand>,
    mut target_camera_zoom: ResMut<Settings>,
) {
    if let Some(Ok(LoadRadiusCommand { radius })) = log.take() {
        target_camera_zoom.load_radius = radius;
    }
}
