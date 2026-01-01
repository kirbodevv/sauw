use bevy::prelude::*;

mod constants;
mod core;
mod player;
mod plugins;
mod world;

use plugins::game::GamePlugin;

use crate::world::{
    block::{BlockId, Layer},
    block_registry::BlockRegistry,
    world::{ChunkPos, World},
};

fn main() {
    let registry = BlockRegistry::new();
    let mut world = World::new();

    let chunk_pos = ChunkPos { x: 0, y: 0 };
    let chunk = world.get_chunk_mut(chunk_pos);

    chunk.set(3, 4, Layer::Object, BlockId(2)); // tree

    let block = chunk.get(3, 4, Layer::Object);
    println!("Block at (3,4): {}", registry.get(block).name);

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .run();
}
