use bevy::prelude::*;

use crate::{
    constants::{GROUND_LAYER, OBJECT_LAYER},
    game::{
        GameState,
        assets::resource::AtlasAssetsParam,
        registry::block_registry::{BlockId, BlockRegistry},
        world::{
            ChunkCoord,
            chunk_manager::{ChunkBlocksStore, ChunkManager},
            idx_2d,
            render::{
                BlockEntity, BlockPos, ChunkMesh, chunk_mesh::build_ground_mesh, chunk_spawner,
            },
        },
    },
    shared::RenderParam,
};

#[derive(Message)]
pub struct SetBlock {
    pub id: BlockId,
    pub x: i32,
    pub y: i32,
    pub layer: u8,
}

#[derive(Message)]
pub struct RebuildGroundMesh {
    pub chunk_coord: ChunkCoord,
}

#[derive(Message)]
pub struct DespawnBlock {
    pub chunk_coord: ChunkCoord,
    pub pos: BlockPos,
}

#[derive(Message)]
pub struct SpawnBlock {
    pub id: BlockId,
    pub chunk_coord: ChunkCoord,
    pub pos: BlockPos,
}

pub fn set_block(
    mut reader: MessageReader<SetBlock>,
    mut blocks_store: ResMut<ChunkBlocksStore>,
    mut rebuild_writer: MessageWriter<RebuildGroundMesh>,
    mut spawn_writer: MessageWriter<SpawnBlock>,
    mut despawn_writer: MessageWriter<DespawnBlock>,
) {
    for event in reader.read() {
        let SetBlock { id, x, y, layer } = *event;

        let chunk_coord = ChunkCoord::from_world_block_pos(x, y);

        let Some(blocks) = blocks_store.get_mut(&chunk_coord) else {
            continue;
        };

        match layer as usize {
            GROUND_LAYER => {
                blocks.ground[idx_2d(x as usize, y as usize)] = id;
                rebuild_writer.write(RebuildGroundMesh { chunk_coord });
            }
            OBJECT_LAYER => {
                let old = blocks.objects[idx_2d(x as usize, y as usize)];

                if !old.is_air() {
                    despawn_writer.write(DespawnBlock {
                        chunk_coord,
                        pos: BlockPos::from_world_block_pos(x, y, layer),
                    });
                }

                spawn_writer.write(SpawnBlock {
                    id,
                    chunk_coord,
                    pos: BlockPos::from_world_block_pos(x, y, layer),
                });

                blocks.objects[idx_2d(x as usize, y as usize)] = id;
            }
            _ => continue,
        }
    }
}

pub fn spawn_block(
    mut commands: Commands,
    mut reader: MessageReader<SpawnBlock>,
    assets: AtlasAssetsParam,
    manager: Res<ChunkManager>,
    blocks: Res<BlockRegistry>,
) {
    for event in reader.read() {
        let SpawnBlock {
            id,
            chunk_coord,
            pos,
        } = event;
        if id.is_air() {
            continue;
        }

        let Some(entity) = manager.entity(&chunk_coord) else {
            continue;
        };

        let Ok(mut parent) = commands.get_entity(entity) else {
            continue;
        };

        parent.with_children(|parent| {
            chunk_spawner::spawn_block(parent, blocks.get(*id), *pos, *chunk_coord, &assets);
        });
    }
}

pub fn despawn_block(
    mut reader: MessageReader<DespawnBlock>,
    mut commands: Commands,
    query: Query<(Entity, &BlockPos, &ChunkCoord), With<BlockEntity>>,
) {
    for event in reader.read() {
        for (entity, block_pos, chunk_coord) in &query {
            if *chunk_coord == event.chunk_coord && *block_pos == event.pos {
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}

pub fn rebuild_ground_mesh(
    blocks_store: Res<ChunkBlocksStore>,
    registry: Res<BlockRegistry>,
    atlas_assets: AtlasAssetsParam,
    mut render: RenderParam,
    mut query: Query<(&ChunkMesh, &mut Mesh2d)>,
    mut reader: MessageReader<RebuildGroundMesh>,
) {
    for event in reader.read() {
        let Some(blocks) = blocks_store.get(&event.chunk_coord) else {
            continue;
        };

        for (chunk_mesh, mut mesh2d) in &mut query {
            if chunk_mesh.coord != event.chunk_coord {
                continue;
            }

            let new_mesh = build_ground_mesh(blocks, &registry, atlas_assets.block_atlas());

            let mesh_handle = render.add_mesh(new_mesh);
            mesh2d.0 = mesh_handle;

            break;
        }
    }
}

pub struct ChunkUpdatePlugin;

impl Plugin for ChunkUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RebuildGroundMesh>()
            .add_message::<SetBlock>()
            .add_message::<DespawnBlock>()
            .add_message::<SpawnBlock>()
            .add_systems(
                Update,
                (rebuild_ground_mesh, set_block, despawn_block, spawn_block)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
