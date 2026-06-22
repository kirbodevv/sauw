use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::{
    constants::TILE_SIZE,
    game::{
        drop::{DroppedBy, SpawnDrop},
        player::Player,
        registry::item_registry::ItemRegistry,
    },
    shared::IntOrFloat,
};

#[derive(Parser, ConsoleCommand)]
#[command(name = "drop")]
pub struct DropCommand {
    pub item: String,
    pub count: Option<u32>,
    pub x: Option<IntOrFloat>,
    pub y: Option<IntOrFloat>,
}

pub fn drop(
    mut log: ConsoleCommand<DropCommand>,
    mut event_writer: MessageWriter<SpawnDrop>,
    player: Single<&mut Transform, With<Player>>,
    item_registry: Option<Res<ItemRegistry>>,
) {
    if let Some(Ok(DropCommand { item, count, x, y })) = log.take() {
        if let Some(registry) = item_registry {
            let id = registry.try_id_by_name(&item);

            let Some(id) = id else {
                log.reply_failed(format!("item not found: {}", item));
                return;
            };

            let count = count.unwrap_or(1);

            let position = Vec2::new(
                x.map(|x| x.to_f32())
                    .unwrap_or(player.translation.x / TILE_SIZE),
                y.map(|y| y.to_f32())
                    .unwrap_or(player.translation.y / TILE_SIZE),
            );
            event_writer.write(SpawnDrop {
                id,
                count,
                position,
                dropped_by: DroppedBy::Command,
            });
        }
    }
}
