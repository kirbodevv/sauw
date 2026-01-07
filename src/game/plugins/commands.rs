use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

use crate::game::commands::{
    camzoom::{CamZoomCommand, cam_zoom_command},
    tp::{TpCommand, tp_command},
};

pub struct GameCommandsPlugin;

impl Plugin for GameCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() })
            .add_console_command::<TpCommand, _>(tp_command)
            .add_console_command::<CamZoomCommand, _>(cam_zoom_command);
    }
}
