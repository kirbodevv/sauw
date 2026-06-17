use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::{
    constants::TILE_SIZE,
    game::{drop::spawner::SpawnDrop, player::Player, registry::item_registry::ItemRegistry},
};

#[derive(Clone)]
pub enum IntOrFloat {
    Int(i32),
    Float(f32),
}

impl IntOrFloat {
    pub fn to_f32(self) -> f32 {
        match self {
            IntOrFloat::Int(i) => i as f32 + 0.5,
            IntOrFloat::Float(f) => f,
        }
    }
}

impl std::str::FromStr for IntOrFloat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('.') {
            if let Ok(i) = s.parse::<i32>() {
                return Ok(IntOrFloat::Int(i));
            }
        }
        s.parse::<f32>()
            .map(IntOrFloat::Float)
            .map_err(|e| e.to_string())
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "drop")]
pub struct DropCommand {
    pub item: String,
    pub x: Option<IntOrFloat>,
    pub y: Option<IntOrFloat>,
}

pub fn drop(
    mut log: ConsoleCommand<DropCommand>,
    mut event_writer: MessageWriter<SpawnDrop>,
    player: Single<&mut Transform, With<Player>>,
    item_registry: Option<Res<ItemRegistry>>,
) {
    if let Some(Ok(DropCommand { item, x, y })) = log.take() {
        if let Some(registry) = item_registry {
            event_writer.write(SpawnDrop {
                id: registry.id_by_name(&item),
                position: Vec2::new(
                    x.map(|x| x.to_f32())
                        .unwrap_or(player.translation.x / TILE_SIZE),
                    y.map(|y| y.to_f32())
                        .unwrap_or(player.translation.y / TILE_SIZE),
                ),
            });
        }
    }
}
