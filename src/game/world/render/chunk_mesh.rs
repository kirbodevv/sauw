use bevy::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{
        assets::atlas::AtlasAsset,
        registry::block_registry::BlockRegistry,
        world::{
            chunk_positions,
            types::{ChunkBlocks, idx_2d},
        },
    },
    shared::MeshBuilder,
};

pub fn build_ground_mesh(
    blocks: &ChunkBlocks,
    registry: &BlockRegistry,
    atlas: &AtlasAsset,
) -> Mesh {
    let mut builder = MeshBuilder::default();

    chunk_positions().for_each(|(x, y)| {
        let id = blocks.ground[idx_2d(x, y)];
        let block = registry.get(id);
        let position = Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE;
        let size = block.sprite_size;
        let offset = block.sprite_offset;
        if let Some(texture_id) = block.texture_id {
            builder.append_quad(texture_id, atlas, position, size, offset);
        }
    });

    builder.build()
}
