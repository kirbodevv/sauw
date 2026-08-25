use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::world::config::WorldConfig;

#[derive(Parser, ConsoleCommand)]
#[command(name = "loadradius")]
pub struct LoadRadiusCommand {
    radius: i32,
}

pub fn load_radius(mut log: ConsoleCommand<LoadRadiusCommand>, mut config: ResMut<WorldConfig>) {
    if let Some(Ok(LoadRadiusCommand { radius })) = log.take() {
        config.load_radius = radius;
    }
}
