use bevy::{prelude::*, sprite::Anchor};
use bevy_firefly::{
    occluders::Occluder2d,
    sprites::{NormalMap, SpriteHeight},
};
use bevy_rapier2d::dynamics::RigidBody;

use crate::{
    constants::{OBJECT_LAYER, TILE_SIZE},
    game::{
        GameState,
        assets::resource::AtlasAssetsParam,
        registry::block_registry::{BlockDefinition, BlockRegistry},
        world::{
            Chunk, ChunkCoord,
            chunk_manager::ChunkManager,
            chunk_positions,
            render::{
                YSort,
                chunk_mesh::build_ground_mesh,
                components::{BlockEntity, BlockPos},
                y_sort_z,
            },
            types::{ChunkBlocks, idx_2d},
        },
    },
    shared::RenderParam,
};

#[derive(Message)]
pub struct SpawnChunk {
    pub chunk_coord: ChunkCoord,
    pub blocks: ChunkBlocks,
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
    for SpawnChunk {
        chunk_coord,
        blocks,
    } in reader.read()
    {
        let pos = chunk_coord.to_world_pos();

        let id = commands
            .spawn((
                Chunk,
                *chunk_coord,
                Visibility::default(),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .with_children(|parent| {
                spawn_ground(parent, blocks, &registry, &atlas_assets, &mut render_param);
                spawn_objects(parent, blocks, &registry, &atlas_assets, pos.y);
            })
            .id();

        manager.entities.insert(*chunk_coord, id);
    }
}

fn spawn_ground(
    parent: &mut ChildSpawnerCommands<'_>,
    blocks: &ChunkBlocks,
    registry: &BlockRegistry,
    atlas_assets: &AtlasAssetsParam,
    render_param: &mut RenderParam,
) {
    let ground_mesh = build_ground_mesh(blocks, registry, atlas_assets.block_atlas());

    parent.spawn((
        Mesh2d(render_param.add_mesh(ground_mesh)),
        MeshMaterial2d(render_param.add_material(atlas_assets.block_texture(), None)),
        Transform::default(),
    ));
}

fn spawn_objects(
    parent: &mut ChildSpawnerCommands<'_>,
    blocks: &ChunkBlocks,
    registry: &BlockRegistry,
    atlas_assets: &AtlasAssetsParam,
    chunk_world_y: f32,
) {
    for (x, y) in chunk_positions() {
        let block = blocks.objects[idx_2d(x, y)];

        if block.is_air() {
            continue;
        }

        let block = registry.get(block);

        spawn_block(
            parent,
            block,
            BlockPos::new(x as u8, y as u8, OBJECT_LAYER as u8),
            atlas_assets,
            chunk_world_y,
        );
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

fn spawn_block(
    parent: &mut ChildSpawnerCommands<'_>,
    block: &BlockDefinition,
    pos: BlockPos,
    assets: &AtlasAssetsParam,
    chunk_world_y: f32,
) {
    let local_x = pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let local_y = pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let atlas_entry = assets.block_atlas().get(block.texture_id.unwrap());
    let padding = 0.5;
    let sprite_rect = atlas_entry.rect_with_padding(padding);

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
                image: assets.block_texture().clone(),
                rect: Some(sprite_rect),
                custom_size: Some(block.sprite_size),
                ..default()
            },
            Anchor::CENTER,
            NormalMap::from_image(assets.block_normal_texture().clone()),
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
