use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::world::{chunk_manager::ChunkManager, render::chunk_update::RebuildGroundMesh};

#[derive(Parser, ConsoleCommand)]
#[command(name = "rebuild")]
pub struct RebuildMeshesCommand;

pub fn rebuild_meshes(
    mut log: ConsoleCommand<RebuildMeshesCommand>,
    manager: Res<ChunkManager>,
    mut writer: MessageWriter<RebuildGroundMesh>,
) {
    if let Some(Ok(RebuildMeshesCommand)) = log.take() {
        for chunk_coord in manager.spawned_chunks() {
            writer.write(RebuildGroundMesh { chunk_coord });
        }
    }
}
