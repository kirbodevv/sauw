use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

mod camzoom;
mod debug;
mod load_radius;
mod tp;

use camzoom::*;
use debug::*;
use load_radius::*;
use tp::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() })
            .add_console_command::<TpCommand, _>(tp)
            .add_console_command::<CamZoomCommand, _>(cam_zoom)
            .add_console_command::<LoadRadiusCommand, _>(load_radius)
            .add_console_command::<DebugCommand, _>(debug);
    }
}
