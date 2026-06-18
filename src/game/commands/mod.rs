use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

mod camzoom;
mod craft;
mod damage;
mod debug;
mod drop;
mod heal;
mod inventory;
mod load_radius;
mod safe_zone;
mod time;
mod tp;

use camzoom::*;
use craft::*;
use damage::*;
use debug::*;
use drop::*;
use heal::*;
use inventory::*;
use load_radius::*;
use safe_zone::*;
use time::*;
use tp::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() })
            .add_console_command::<TpCommand, _>(tp)
            .add_console_command::<CamZoomCommand, _>(cam_zoom)
            .add_console_command::<LoadRadiusCommand, _>(load_radius)
            .add_console_command::<DebugCommand, _>(debug)
            .add_console_command::<DamageCommand, _>(damage)
            .add_console_command::<HealCommand, _>(heal)
            .add_console_command::<InventoryCommand, _>(inventory)
            .add_console_command::<TimeCommand, _>(time)
            .add_console_command::<SafeZoneCommand, _>(safe_zone)
            .add_console_command::<CraftCommand, _>(craft)
            .add_console_command::<DropCommand, _>(drop);
    }
}
