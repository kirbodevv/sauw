use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::player::Player;

#[derive(Parser, ConsoleCommand)]
#[command(name = "tp")]
pub struct TpCommand {
    x: i32,
    y: i32,
}

pub fn tp(mut log: ConsoleCommand<TpCommand>, mut query: Query<&mut Transform, With<Player>>) {
    if let Some(Ok(TpCommand { x, y })) = log.take() {
        for mut transform in &mut query {
            transform.translation = Vec3::new(x as f32, y as f32, 50.0);
        }
    }
}
