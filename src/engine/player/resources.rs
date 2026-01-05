use crate::engine::world::components::ChunkCoord;
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct CurrentPlayerChunk(pub Option<ChunkCoord>);
