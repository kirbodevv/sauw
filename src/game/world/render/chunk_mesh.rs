use bevy::prelude::*;

use crate::{
    constants::{CHUNK_VOLUME, GROUND_LAYER, TILE_SIZE},
    game::{
        assets::atlas::{AtlasAsset, TextureId},
        registry::block_registry::{BlockId, BlockRegistry},
        world::{chunk_positions, types::idx},
    },
    shared::MeshBuilder,
};

pub fn build_ground_mesh(
    blocks: &[BlockId; CHUNK_VOLUME],
    registry: &BlockRegistry,
    atlas: &AtlasAsset,
) -> Mesh {
    let mut builder = MeshBuilder::default();

    chunk_positions().for_each(|(x, y)| {
        let id = blocks[idx(x, y, GROUND_LAYER)];
        let block = registry.get(id);
        let position = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
        let size = block.sprite_size;
        let offset = block.sprite_offset;

        builder.append_quad(TextureId::new(block.name), atlas, position, size, offset);
    });

    builder.build()
}
