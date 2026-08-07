use bevy::prelude::*;

pub mod chunk_mesh;
pub mod chunk_spawner;
pub mod y_sort;

pub use y_sort::*;

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((chunk_spawner::ChunkSpawnerPlugin, y_sort::YSortPlugin));
    }
}
