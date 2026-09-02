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
                ChunkMesh, YSort,
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

        let entity = commands
            .spawn((
                Chunk,
                *chunk_coord,
                Visibility::default(),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .id();

        spawn_ground(
            &mut commands,
            entity,
            &chunk_coord,
            blocks,
            &registry,
            &atlas_assets,
            &mut render_param,
        );
        spawn_objects(
            &mut commands,
            entity,
            blocks,
            &registry,
            &atlas_assets,
            *chunk_coord,
        );

        manager.register(*chunk_coord, entity);
    }
}

fn spawn_ground(
    commands: &mut Commands,
    parent: Entity,
    chunk_coord: &ChunkCoord,
    blocks: &ChunkBlocks,
    registry: &BlockRegistry,
    atlas_assets: &AtlasAssetsParam,
    render_param: &mut RenderParam,
) {
    let ground_mesh = build_ground_mesh(blocks, registry, atlas_assets.block_atlas());

    commands.entity(parent).with_child((
        ChunkMesh {
            coord: *chunk_coord,
        },
        Mesh2d(render_param.add_mesh(ground_mesh)),
        MeshMaterial2d(render_param.add_material(atlas_assets.block_texture(), None)),
        Transform::default(),
    ));
}

pub fn spawn_objects(
    commands: &mut Commands,
    parent: Entity,
    blocks: &ChunkBlocks,
    registry: &BlockRegistry,
    atlas_assets: &AtlasAssetsParam,
    chunk_coord: ChunkCoord,
) {
    commands.entity(parent).with_children(|parent| {
        for (x, y) in chunk_positions() {
            let block = blocks.objects[idx_2d(x, y)];

            if block.is_air() {
                continue;
            }

            let block = registry.get(block);
            let pos = BlockPos::new(x as u8, y as u8, OBJECT_LAYER as u8);
            spawn_block(parent, block, pos, chunk_coord, atlas_assets);
        }
    });
}

pub fn despawn_chunk(
    mut commands: Commands,
    mut reader: MessageReader<DespawnChunk>,
    mut manager: ResMut<ChunkManager>,
) {
    for chunk in reader.read() {
        if let Some(entity) = manager.unregister(&chunk.chunk_coord) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_block(
    parent: &mut ChildSpawnerCommands<'_>,
    block: &BlockDefinition,
    pos: BlockPos,
    chunk_coord: ChunkCoord,
    assets: &AtlasAssetsParam,
) {
    let local_x = pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let local_y = pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let atlas_entry = assets.block_atlas().get(block.texture_id.unwrap());
    let padding = 0.5;
    let sprite_rect = atlas_entry.rect_with_padding(padding);

    let chunk_world_y = chunk_coord.to_world_pos().y;

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
        chunk_coord,
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
