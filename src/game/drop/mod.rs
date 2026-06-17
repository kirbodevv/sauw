use bevy::prelude::*;

pub mod spawner;

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spawner::DropSpawnerPlugin,));
    }
}
