use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use bevy_rapier2d::prelude::*;
use clap::Parser;

#[derive(Parser, ConsoleCommand)]
#[command(name = "debug")]
pub struct DebugCommand;

pub fn debug(mut log: ConsoleCommand<DebugCommand>, mut debug_context: ResMut<DebugRenderContext>) {
    if let Some(Ok(DebugCommand)) = log.take() {
        debug_context.enabled = !debug_context.enabled;
    }
}
