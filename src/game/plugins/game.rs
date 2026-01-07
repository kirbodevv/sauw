#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/load_textures.rs"));

use crate::game::{
    player::{
        resources::CurrentPlayerChunk,
        systems::{player_animate, player_movement, spawn_player},
    },
    plugins::startup::StartupSet,
    rendering::{TargetCameraZoom, camera_follow, spawn_camera, y_sort, zoom_camera},
    resources::{GameRegistry, Textures},
    world::{
        resources::{LoadedChunks, Settings, WorldSeed},
        systems::manage_chunks,
    },
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(GameRegistry::new())
            .insert_resource(Textures::new())
            .insert_resource(LoadedChunks::new())
            .insert_resource(CurrentPlayerChunk(None))
            .insert_resource(WorldSeed(0))
            .insert_resource(TargetCameraZoom(1.0))
            .insert_resource(Settings { load_radius: 2 })
            .add_systems(Startup, load_textures.in_set(StartupSet::Assets))
            .add_systems(
                Startup,
                (spawn_camera, spawn_player).in_set(StartupSet::Actors),
            )
            .configure_sets(Startup, (StartupSet::Assets, StartupSet::Actors).chain())
            .add_systems(
                Update,
                (
                    player_movement,
                    player_animate,
                    camera_follow,
                    zoom_camera,
                    manage_chunks,
                    y_sort,
                )
                    .chain(),
            );
    }
}
