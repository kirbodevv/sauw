use bevy::prelude::*;

use crate::{
    constants::{GROUND_LAYER, OBJECT_LAYER},
    game::{
        GameState,
        assets::resource::AtlasAssetsParam,
        registry::block_registry::{BlockId, BlockRegistry},
        world::{
            Chunk, ChunkCoord,
            chunk_manager::{ChunkBlocksStore, DirtyChunks},
            idx_2d,
            render::{BlockEntity, ChunkMesh, chunk_mesh::build_ground_mesh, chunk_spawner},
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

pub fn set_block(
    mut reader: MessageReader<SetBlock>,
    mut blocks_store: ResMut<ChunkBlocksStore>,
    mut dirty: ResMut<DirtyChunks>,
) {
    for event in reader.read() {
        let chunk_coord = ChunkCoord::from_world_block_pos(event.x, event.y);

        let Some(blocks) = blocks_store.get_mut(&chunk_coord) else {
            continue;
        };

        let idx = idx_2d(event.x as usize, event.y as usize);

        match event.layer as usize {
            GROUND_LAYER => {
                blocks.ground[idx] = event.id;
                dirty.ground.insert(chunk_coord);
            }

            OBJECT_LAYER => {
                blocks.objects[idx] = event.id;
                dirty.objects.insert(chunk_coord);
            }

            _ => {}
        }
    }
}

pub fn sync_objects(
    mut commands: Commands,
    mut dirty: ResMut<DirtyChunks>,

    blocks_store: Res<ChunkBlocksStore>,
    registry: Res<BlockRegistry>,
    atlas_assets: AtlasAssetsParam,

    chunks: Query<(Entity, &ChunkCoord), With<Chunk>>,
    blocks: Query<(Entity, &ChunkCoord), With<BlockEntity>>,
) {
    for chunk_coord in dirty.objects.drain() {
        let Some((chunk_entity, _)) = chunks.iter().find(|(_, coord)| **coord == chunk_coord)
        else {
            continue;
        };

        for (entity, block_chunk) in &blocks {
            if *block_chunk == chunk_coord {
                commands.entity(entity).despawn();
            }
        }

        let Some(chunk_blocks) = blocks_store.get(&chunk_coord) else {
            continue;
        };

        chunk_spawner::spawn_objects(
            &mut commands,
            chunk_entity,
            chunk_blocks,
            &registry,
            &atlas_assets,
            chunk_coord,
        );
    }
}

pub fn rebuild_ground_mesh(
    mut dirty: ResMut<DirtyChunks>,
    blocks_store: Res<ChunkBlocksStore>,
    registry: Res<BlockRegistry>,
    atlas_assets: AtlasAssetsParam,
    mut render: RenderParam,
    mut query: Query<(&ChunkMesh, &mut Mesh2d)>,
) {
    for chunk_coord in dirty.ground.drain() {
        let Some(blocks) = blocks_store.get(&chunk_coord) else {
            continue;
        };

        for (chunk_mesh, mut mesh2d) in &mut query {
            if chunk_mesh.coord != chunk_coord {
                continue;
            }

            let new_mesh = build_ground_mesh(blocks, &registry, atlas_assets.block_atlas());

            mesh2d.0 = render.add_mesh(new_mesh);

            break;
        }
    }
}

pub struct ChunkUpdatePlugin;

impl Plugin for ChunkUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirtyChunks>()
            .add_message::<SetBlock>()
            .add_systems(
                Update,
                (rebuild_ground_mesh, set_block, sync_objects)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
