use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::{registry::block_registry::BlockRegistry, world::render::chunk_update::SetBlock};

#[derive(Parser, ConsoleCommand)]
#[command(name = "setblock")]
pub struct SetBlockCommand {
    pub x: i32,
    pub y: i32,
    pub layer: i32,
    pub block: String,
}

pub fn setblock(
    mut log: ConsoleCommand<SetBlockCommand>,
    mut writer: MessageWriter<SetBlock>,
    blocks: Res<BlockRegistry>,
) {
    if let Some(Ok(SetBlockCommand { x, y, layer, block })) = log.take() {
        let Some(id) = blocks.try_id_by_name(&block) else {
            log.reply_failed("block not found");
            return;
        };

        writer.write(SetBlock {
            x: x as i32,
            y: y as i32,
            layer: layer as u8,
            id,
        });
    }
}
