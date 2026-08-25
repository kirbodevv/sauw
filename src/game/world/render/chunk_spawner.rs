use bevy::{prelude::*, sprite::Anchor};
use bevy_firefly::{
    occluders::Occluder2d,
    sprites::{NormalMap, SpriteHeight},
};
use bevy_rapier2d::dynamics::RigidBody;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, OBJECT_LAYER, TILE_SIZE},
    game::{
        GameState,
        assets::{
            atlas::{AtlasAsset, TextureId},
            resource::AtlasAssetsParam,
        },
        registry::block_registry::{BlockDefinition, BlockId, BlockRegistry},
        world::{
            Chunk, ChunkCoord,
            chunk_manager::ChunkManager,
            generator::idx,
            render::{
                YSort,
                chunk_mesh::build_ground_mesh,
                components::{BlockEntity, BlockPos},
                y_sort_z,
            },
        },
    },
    shared::RenderParam,
};

#[derive(Message)]
pub struct SpawnChunk {
    pub chunk_coord: ChunkCoord,
    pub blocks: [BlockId; CHUNK_VOLUME],
}

#[derive(Message)]
pub struct DespawnChunk {
    pub chunk_coord: ChunkCoord,
}

pub fn spawn_chunk(
    mut commands: Commands,
    registry: Res<BlockRegistry>,
    mut reader: MessageReader<SpawnChunk>,
    mut render_param: RenderParam,
    atlas_assets: AtlasAssetsParam,
    mut manager: ResMut<ChunkManager>,
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
            let ground_mesh =
                build_ground_mesh(&chunk.blocks, &registry, atlas_assets.block_atlas());

            parent.spawn((
                Mesh2d(render_param.add_mesh(ground_mesh)),
                MeshMaterial2d(render_param.add_material(atlas_assets.block_texture(), None)),
                Transform::default(),
            ));

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block = chunk.blocks[idx(x, y, OBJECT_LAYER)];
                    if block == air {
                        continue;
                    }
                    let block = registry.get(block);
                    spawn_block(
                        parent,
                        block,
                        BlockPos::new(x as u8, y as u8, OBJECT_LAYER as u8),
                        &atlas_assets.image_assets.block,
                        &atlas_assets.image_assets.block_normal,
                        atlas_assets
                            .atlases
                            .get(atlas_assets.atlas_assets.block.id())
                            .unwrap(),
                        chunk_world_y,
                    );
                }
            }
        });
        manager
            .entities
            .insert(chunk.chunk_coord, chunk_entity.id());
    }
}

pub fn despawn_chunk(
    mut commands: Commands,
    mut reader: MessageReader<DespawnChunk>,
    mut manager: ResMut<ChunkManager>,
) {
    for chunk in reader.read() {
        if let Some(entity) = manager.entities.remove(&chunk.chunk_coord) {
            commands.entity(entity).despawn();
        }
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

pub struct ChunkSpawnerPlugin;

impl Plugin for ChunkSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnChunk>()
            .add_message::<DespawnChunk>()
            .add_systems(
                Update,
                (despawn_chunk, spawn_chunk)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
