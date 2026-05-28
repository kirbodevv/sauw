use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        atlas::{Atlas, TextureId},
        registry::{GameRegistry, block_registry::BlockId},
        world::generator::idx,
    },
};

pub fn build_chunk_mesh(
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &GameRegistry,
    atlas: &Atlas,
) -> Mesh {
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    for layer in 0..2 {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let block_id = chunk_blocks[idx(x, y, layer)];
                let block = registry.blocks.get(block_id);

                if block.name == "air" {
                    continue;
                }

                let texture_id = TextureId::new(block.name);
                let atlas_entry = &atlas.entries[&texture_id];

                let px = x as f32 * TILE_SIZE;
                let py = y as f32 * TILE_SIZE;
                let size = block.sprite_size;
                let offset = block.sprite_offset;

                let tex_x = atlas_entry.x() as f32;
                let tex_y = atlas_entry.y() as f32;
                let tex_w = atlas_entry.width() as f32;
                let tex_h = atlas_entry.height() as f32;

                let atlas_texture_w = atlas.width as f32;
                let atlas_texture_h = atlas.height as f32;

                let pad = 0.5;

                let u0 = (tex_x + pad) / atlas_texture_w;
                let v0 = (tex_y + pad) / atlas_texture_h;
                let u1 = (tex_x + tex_w - pad) / atlas_texture_w;
                let v1 = (tex_y + tex_h - pad) / atlas_texture_h;

                let base = positions.len() as u32;

                let cx = px + TILE_SIZE / 2.0 + offset.x;
                let cy = py + TILE_SIZE / 2.0 + offset.y;

                let hw = size.x / 2.0;
                let hh = size.y / 2.0;

                positions.push([cx - hw, cy - hh, 0.0]);
                positions.push([cx + hw, cy - hh, 0.0]);
                positions.push([cx - hw, cy + hh, 0.0]);
                positions.push([cx + hw, cy + hh, 0.0]);

                uvs.push([u0, v1]);
                uvs.push([u1, v1]);
                uvs.push([u0, v0]);
                uvs.push([u1, v0]);

                indices.extend_from_slice(&[
                    base,
                    base + 1,
                    base + 2,
                    base + 2,
                    base + 1,
                    base + 3,
                ]);
            }
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn spawn_chunk_mesh(
    parent: &mut ChildSpawnerCommands<'_>,
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &GameRegistry,
    block_texture: &Handle<Image>,
    block_atlas: &Atlas,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let terrain_mesh = build_chunk_mesh(chunk_blocks, registry, block_atlas);

    parent.spawn((
        Mesh2d(meshes.add(terrain_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(block_texture.clone()),
            ..default()
        })),
    ));
}
