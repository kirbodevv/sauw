use crate::game::commands::*;
use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

pub struct GameCommandsPlugin;

impl Plugin for GameCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() })
            .add_console_command::<TpCommand, _>(tp)
            .add_console_command::<CamZoomCommand, _>(cam_zoom)
            .add_console_command::<LoadRadiusCommand, _>(load_radius);
    }
}
