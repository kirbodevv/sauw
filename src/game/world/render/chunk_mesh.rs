use bevy::prelude::*;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        assets::atlas::{AtlasAsset, TextureId},
        registry::block_registry::{BlockId, BlockRegistry},
        world::generator::idx,
    },
    shared::{MeshBuilder, RenderParam},
};

pub fn build_ground_mesh(
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &BlockRegistry,
    atlas: &AtlasAsset,
) -> Mesh {
    let mut mesh_builder = MeshBuilder::default();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let block_id = chunk_blocks[idx(x, y, 0)];
            let block = registry.get(block_id);

            if block.name == "air" {
                continue;
            }

            let position = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            let size = block.sprite_size;
            let offset = block.sprite_offset;

            mesh_builder.append_quad(TextureId::new(block.name), atlas, position, size, offset);
        }
    }
    mesh_builder.build()
}

pub fn spawn_chunk_mesh(
    parent: &mut ChildSpawnerCommands<'_>,
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &BlockRegistry,
    block_texture: &Handle<Image>,
    block_atlas: &AtlasAsset,
    render_param: &mut RenderParam,
) {
    let material = render_param.materials.add(ColorMaterial {
        texture: Some(block_texture.clone()),
        ..default()
    });

    let ground_mesh = build_ground_mesh(chunk_blocks, registry, block_atlas);
    parent.spawn((
        Mesh2d(render_param.meshes.add(ground_mesh)),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
