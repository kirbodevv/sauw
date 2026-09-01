use bevy::prelude::*;

use crate::{
    game::{
        GameState,
        assets::resource::AtlasAssetsParam,
        registry::block_registry::BlockRegistry,
        world::{
            ChunkCoord,
            chunk_manager::ChunkBlocksStore,
            render::{ChunkMesh, chunk_mesh::build_ground_mesh},
        },
    },
    shared::RenderParam,
};

#[derive(Message)]
pub struct RebuildGroundMesh {
    pub chunk_coord: ChunkCoord,
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
        let Some(blocks) = blocks_store.blocks.get(&event.chunk_coord) else {
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
        app.add_message::<RebuildGroundMesh>().add_systems(
            Update,
            rebuild_ground_mesh.run_if(in_state(GameState::Gaming)),
        );
    }
}
