use bevy::prelude::*;
use bevy_firefly::occluders::Occluder2d;
use bevy_rapier2d::dynamics::RigidBody;

use crate::{
    constants::{CHUNK_SIZE, TILE_SIZE},
    game::{
        ImageAssets,
        assets::atlas::Atlas,
        registry::{block_registry::BlockDefinition, block_registry::BlockRegistry},
        world::{
            BlockEntity, BlockPos, Chunk,
            chunk_mesh::spawn_chunk_mesh,
            generator::{GeneratedChunk, idx},
        },
    },
};

pub fn spawn_chunk(
    registry: Res<BlockRegistry>,
    mut commands: Commands,
    mut reader: MessageReader<GeneratedChunk>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    atlases: Res<Assets<Atlas>>,
    image_assets: Res<ImageAssets>,
) {
    let air = registry.id_by_name("air");
    for chunk in reader.read() {
        let chunk_world_x = chunk.chunk_coord.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE;
        let chunk_world_y = chunk.chunk_coord.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE;

        let mut chunk_entity = commands.spawn((
            Chunk,
            chunk.chunk_coord,
            Visibility::default(),
            Transform::from_xyz(chunk_world_x, chunk_world_y, 0.0),
        ));

        chunk_entity.with_children(|parent| {
            spawn_chunk_mesh(
                parent,
                &chunk.blocks,
                &registry,
                &image_assets.atlas_block_texture,
                &atlases.get(image_assets.atlas_block.id()).unwrap(),
                &mut meshes,
                &mut materials,
                chunk_world_y,
            );

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block = chunk.blocks[idx(x, y, 1)];
                    if block == air {
                        continue;
                    }
                    let block = registry.get(block);
                    spawn_block(parent, block, BlockPos::new(x as u8, y as u8, 1));
                }
            }
        });
    }
}

pub fn spawn_block(parent: &mut ChildSpawnerCommands<'_>, block: &BlockDefinition, pos: BlockPos) {
    let local_x = pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let local_y = pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let is_object = pos.layer == 1;

    if is_object {
        let mut entity = parent.spawn((
            Transform::from_xyz(local_x, local_y, pos.layer as f32),
            BlockEntity,
            RigidBody::Fixed,
            block.collider.clone(),
            pos.clone(),
        ));
        for occluder in &block.occluders {
            entity.with_children(|parent| {
                parent.spawn((
                    Occluder2d::rectangle(occluder.size.x, occluder.size.y),
                    Transform::from_xyz(
                        occluder.offset.x,
                        occluder.offset.y,
                        pos.layer as f32 * TILE_SIZE,
                    ),
                ));
            });
        }
    }
}
