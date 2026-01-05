use bevy::prelude::*;

use crate::engine::{
    player::{
        resources::CurrentPlayerChunk,
        systems::{player_movement, spawn_player},
    },
    plugins::startup::StartupSet,
    rendering::{camera_follow, spawn_camera},
    resources::{BlockTextures, GameRegistry, load_block_textures},
    world::{resources::LoadedChunks, systems::manage_chunks},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(GameRegistry::new())
            .insert_resource(BlockTextures::new())
            .insert_resource(LoadedChunks::new())
            .insert_resource(CurrentPlayerChunk(None))
            .add_systems(Startup, load_block_textures.in_set(StartupSet::Assets))
            .add_systems(
                Startup,
                (spawn_camera, spawn_player).in_set(StartupSet::Actors),
            )
            .configure_sets(Startup, (StartupSet::Assets, StartupSet::Actors).chain())
            .add_systems(
                Update,
                (player_movement, camera_follow, manage_chunks).chain(),
            );
    }
}
