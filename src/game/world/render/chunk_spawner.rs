use bevy::{prelude::*, sprite::Anchor};
use bevy_firefly::{
    occluders::Occluder2d,
    sprites::{NormalMap, SpriteHeight},
};
use bevy_rapier2d::dynamics::RigidBody;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        assets::{
            atlas::{AtlasAsset, TextureId},
            resource::{AtlasAssets, ImageAssets},
        },
        registry::block_registry::{BlockDefinition, BlockId, BlockRegistry},
        world::{
            BlockEntity, BlockPos, Chunk, ChunkCoord,
            generator::idx,
            render::{YSort, chunk_mesh::spawn_chunk_mesh, y_sort_z},
        },
    },
};

#[derive(Message)]
pub struct SpawnChunk {
    pub chunk_coord: ChunkCoord,
    pub blocks: [BlockId; CHUNK_VOLUME],
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_chunk(
    registry: Res<BlockRegistry>,
    mut commands: Commands,
    mut reader: MessageReader<SpawnChunk>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    atlases: Res<Assets<AtlasAsset>>,
    image_assets: Res<ImageAssets>,
    atlas_assets: Res<AtlasAssets>,
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
                &image_assets.block,
                atlases.get(atlas_assets.block.id()).unwrap(),
                &mut meshes,
                &mut materials,
            );

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block = chunk.blocks[idx(x, y, 1)];
                    if block == air {
                        continue;
                    }
                    let block = registry.get(block);
                    spawn_block(
                        parent,
                        block,
                        BlockPos::new(x as u8, y as u8, 1),
                        &image_assets.block,
                        &image_assets.block_normal,
                        atlases.get(atlas_assets.block.id()).unwrap(),
                        chunk_world_y,
                    );
                }
            }
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_block(
    parent: &mut ChildSpawnerCommands<'_>,
    block: &BlockDefinition,
    pos: BlockPos,
    block_texture: &Handle<Image>,
    block_normal_texture: &Handle<Image>,
    block_atlas: &AtlasAsset,
    chunk_world_y: f32,
) {
    let local_x = pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let local_y = pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let atlas_entry = &block_atlas.entries[&TextureId::new(block.name)];
    let padding = 0.5;
    let sprite_rect = Rect::new(
        atlas_entry.x() as f32 + padding,
        atlas_entry.y() as f32 + padding,
        (atlas_entry.x() + atlas_entry.width()) as f32 - padding,
        (atlas_entry.y() + atlas_entry.height()) as f32 - padding,
    );

    let mut entity = parent.spawn((
        Transform::from_xyz(
            local_x,
            local_y,
            y_sort_z(block.y_sort, chunk_world_y + local_y),
        ),
        Visibility::default(),
        YSort { z: block.y_sort },
        BlockEntity,
        RigidBody::Fixed,
        block.collider.clone(),
        pos,
    ));

    entity.with_children(|parent| {
        parent.spawn((
            Sprite {
                image: block_texture.clone(),
                rect: Some(sprite_rect),
                custom_size: Some(block.sprite_size),
                ..default()
            },
            Anchor::CENTER,
            NormalMap::from_image(block_normal_texture.clone()),
            SpriteHeight(0.0),
            Transform::from_translation(block.sprite_offset.extend(0.0)),
        ));

        for occluder in &block.occluders {
            parent.spawn((
                Occluder2d::rectangle(occluder.size.x, occluder.size.y),
                Transform::from_translation(occluder.offset.extend(0.0)),
            ));
        }
    });
}
