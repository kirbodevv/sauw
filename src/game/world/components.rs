use bevy::ecs::component::Component;

#[derive(Component)]
pub struct BlockEntity;

#[derive(Component, Clone)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub layer: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct BelongsToChunk(pub ChunkCoord);
