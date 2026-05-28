use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        atlas::{Atlas, TextureId},
        registry::{
            GameRegistry,
            block_registry::{BlockDefinition, BlockId},
        },
        world::generator::idx,
    },
};

pub fn build_ground_mesh(
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &GameRegistry,
    atlas: &Atlas,
) -> Mesh {
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let block_id = chunk_blocks[idx(x, y, 0)];
            let block = registry.blocks.get(block_id);

            if block.name == "air" {
                continue;
            }

            append_quad(
                block,
                atlas,
                x,
                y,
                0.0,
                &mut positions,
                &mut uvs,
                &mut indices,
            );
        }
    }

    make_mesh(positions, uvs, indices)
}

pub fn build_object_quad(block: &BlockDefinition, atlas: &Atlas) -> Mesh {
    let texture_id = TextureId::new(block.name);
    let atlas_entry = &atlas.entries[&texture_id];

    let size = block.sprite_size;
    let hw = size.x / 2.0;
    let hh = size.y / 2.0;

    let tex_x = atlas_entry.x() as f32;
    let tex_y = atlas_entry.y() as f32;
    let tex_w = atlas_entry.width() as f32;
    let tex_h = atlas_entry.height() as f32;
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;

    let pad = 0.5;
    let u0 = (tex_x + pad) / atlas_w;
    let v0 = (tex_y + pad) / atlas_h;
    let u1 = (tex_x + tex_w - pad) / atlas_w;
    let v1 = (tex_y + tex_h - pad) / atlas_h;

    make_mesh(
        vec![
            [-hw, -hh, 0.0_f32],
            [hw, -hh, 0.0],
            [-hw, hh, 0.0],
            [hw, hh, 0.0],
        ],
        vec![[u0, v1], [u1, v1], [u0, v0], [u1, v0]],
        vec![0u32, 1, 2, 2, 1, 3],
    )
}

pub fn spawn_chunk_mesh(
    parent: &mut ChildSpawnerCommands<'_>,
    chunk_blocks: &[BlockId; CHUNK_VOLUME],
    registry: &GameRegistry,
    block_texture: &Handle<Image>,
    block_atlas: &Atlas,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    chunk_world_y: f32,
) {
    let material = materials.add(ColorMaterial {
        texture: Some(block_texture.clone()),
        ..default()
    });

    let ground_mesh = build_ground_mesh(chunk_blocks, registry, block_atlas);
    parent.spawn((
        Mesh2d(meshes.add(ground_mesh)),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let air = registry.blocks.id_by_name("air");
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let block_id = chunk_blocks[idx(x, y, 1)];
            if block_id == air {
                continue;
            }
            let block = registry.blocks.get(block_id);

            let local_x = x as f32 * TILE_SIZE + TILE_SIZE / 2.0 + block.sprite_offset.x;
            let local_y = y as f32 * TILE_SIZE + TILE_SIZE / 2.0 + block.sprite_offset.y;

            let feet_world_y = chunk_world_y + y as f32 * TILE_SIZE;
            let render_z = 10.0 - feet_world_y * 0.001;

            let quad = build_object_quad(block, block_atlas);
            parent.spawn((
                Mesh2d(meshes.add(quad)),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(local_x, local_y, render_z),
            ));
        }
    }
}

fn append_quad(
    block: &BlockDefinition,
    atlas: &Atlas,
    cx: usize,
    cy: usize,
    z: f32,
    positions: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let texture_id = TextureId::new(block.name);
    let atlas_entry = &atlas.entries[&texture_id];

    let px = cx as f32 * TILE_SIZE;
    let py = cy as f32 * TILE_SIZE;

    let size = block.sprite_size;
    let offset = block.sprite_offset;

    let tex_x = atlas_entry.x() as f32;
    let tex_y = atlas_entry.y() as f32;
    let tex_w = atlas_entry.width() as f32;
    let tex_h = atlas_entry.height() as f32;
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;

    let pad = 0.5;
    let u0 = (tex_x + pad) / atlas_w;
    let v0 = (tex_y + pad) / atlas_h;
    let u1 = (tex_x + tex_w - pad) / atlas_w;
    let v1 = (tex_y + tex_h - pad) / atlas_h;

    let base = positions.len() as u32;

    let cx = px + TILE_SIZE / 2.0 + offset.x;
    let cy = py + TILE_SIZE / 2.0 + offset.y;
    let hw = size.x / 2.0;
    let hh = size.y / 2.0;

    positions.extend_from_slice(&[
        [cx - hw, cy - hh, z],
        [cx + hw, cy - hh, z],
        [cx - hw, cy + hh, z],
        [cx + hw, cy + hh, z],
    ]);
    uvs.extend_from_slice(&[[u0, v1], [u1, v1], [u0, v0], [u1, v0]]);
    indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 1, base + 3]);
}

fn make_mesh(positions: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>, indices: Vec<u32>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
