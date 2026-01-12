pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_WIDTH: f32 = 16.0;
pub const VIEWPORT_WIDTH: f32 = WORLD_WIDTH * TILE_SIZE;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * 2;
pub const CHUNK_WORLD: f32 = TILE_SIZE * CHUNK_SIZE as f32;
