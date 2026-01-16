use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::player::{Player, health::Health};

#[derive(Parser, ConsoleCommand)]
#[command(name = "damage")]
pub struct DamageCommand {
    amount: u8,
}

pub fn damage(
    mut log: ConsoleCommand<DamageCommand>,
    mut player_health: Single<&mut Health, With<Player>>,
) {
    if let Some(Ok(DamageCommand { amount })) = log.take() {
        player_health.damage(amount);
    }
}
