use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::player::{Player, health::Health};

#[derive(Parser, ConsoleCommand)]
#[command(name = "heal")]
pub struct HealCommand {
    amount: u8,
}

pub fn heal(
    mut log: ConsoleCommand<HealCommand>,
    mut player_health: Single<&mut Health, With<Player>>,
) {
    if let Some(Ok(HealCommand { amount })) = log.take() {
        player_health.heal(amount);
    }
}
