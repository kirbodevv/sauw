use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::ui::safe_zone::ChangeSafeZone;

#[derive(Parser, ConsoleCommand)]
#[command(name = "safezone")]
pub struct SafeZoneCommand {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

pub fn safe_zone(
    mut log: ConsoleCommand<SafeZoneCommand>,
    mut safe_zone_writer: MessageWriter<ChangeSafeZone>,
) {
    if let Some(Ok(SafeZoneCommand {
        top,
        bottom,
        left,
        right,
    })) = log.take()
    {
        safe_zone_writer.write(ChangeSafeZone {
            top,
            bottom,
            left,
            right,
        });
    }
}
