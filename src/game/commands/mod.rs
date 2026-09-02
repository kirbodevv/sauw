use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

use crate::game::registry::block_registry::BlockRegistry;

mod camzoom;
mod craft;
mod damage;
mod debug;
mod drop;
mod heal;
mod inventory;
mod load_radius;
mod meshes;
mod safe_zone;
mod setblock;
mod time;
mod tp;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() })
            .add_console_command::<camzoom::CamZoomCommand, _>(camzoom::cam_zoom)
            .add_console_command::<craft::CraftCommand, _>(craft::craft)
            .add_console_command::<damage::DamageCommand, _>(damage::damage)
            .add_console_command::<debug::DebugCommand, _>(debug::debug)
            .add_console_command::<drop::DropCommand, _>(drop::drop)
            .add_console_command::<heal::HealCommand, _>(heal::heal)
            .add_console_command::<inventory::InventoryCommand, _>(inventory::inventory)
            .add_console_command::<load_radius::LoadRadiusCommand, _>(load_radius::load_radius)
            .add_console_command::<meshes::RebuildMeshesCommand, _>(meshes::rebuild_meshes)
            .add_console_command::<safe_zone::SafeZoneCommand, _>(safe_zone::safe_zone)
            .add_console_command::<setblock::SetBlockCommand, _>(
                setblock::setblock.run_if(resource_exists::<BlockRegistry>),
            )
            .add_console_command::<time::TimeCommand, _>(time::time)
            .add_console_command::<tp::TpCommand, _>(tp::tp);
    }
}
