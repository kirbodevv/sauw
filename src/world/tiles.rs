use bevy::prelude::*;

use crate::world::{
    block::{BlockId, Layer},
    block_registry::BlockRegistry,
    chunk::Chunk,
    systems::spawn_chunk,
    textures::load_block_textures,
    world::{ChunkPos, World},
};

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = load_block_textures(&asset_server);
    let registry = BlockRegistry::new();
    let mut game_world = World::new();

    for x in 0..5 {
        for y in 0..5 {
            let pos = ChunkPos { x, y };
            let chunk = game_world.get_chunk_mut(pos);
            fill_chunk_border(chunk, BlockId(2));
            spawn_chunk(&mut commands, &game_world, &registry, &textures, pos);
        }
    }
}

fn fill_chunk_border(chunk: &mut Chunk, block_id: BlockId) {
    let size = 16;

    for x in 0..size {
        chunk.set(x, 0, Layer::Object, block_id);
        chunk.set(x, size - 1, Layer::Object, block_id);
    }

    for y in 1..(size - 1) {
        chunk.set(0, y, Layer::Object, block_id);
        chunk.set(size - 1, y, Layer::Object, block_id);
    }
}
