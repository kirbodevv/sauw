use bevy::prelude::*;

use crate::{
    constants::{CHUNK_VOLUME, TILE_SIZE},
    game::{
        assets::atlas::{AtlasAsset, TextureId},
        registry::block_registry::{BlockId, BlockRegistry},
        world::{chunk_positions, types::idx},
    },
    shared::MeshBuilder,
};

pub fn build_ground_mesh(
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &BlockRegistry,
    atlas: &AtlasAsset,
) -> Mesh {
    let mut builder = MeshBuilder::default();

    for (x, y) in chunk_positions() {
        let block_id = chunk_blocks[idx(x, y, 0)];
        let block = registry.get(block_id);

        if block_id.is_air() {
            continue;
        }

        let position = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
        let size = block.sprite_size;
        let offset = block.sprite_offset;

        builder.append_quad(TextureId::new(block.name), atlas, position, size, offset);
    }

    builder.build()
}
