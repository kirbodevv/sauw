#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/load_textures.rs"));

pub mod commands;
pub mod player;
pub mod registry;
pub mod rendering;
pub mod resources;
pub mod world;

use crate::{
    constants::TILE_SIZE,
    game::{
        commands::CommandsPlugin,
        player::{
            resources::CurrentPlayerChunk,
            systems::{player_animate, player_movement, spawn_player},
        },
        rendering::{TargetCameraZoom, camera_follow, spawn_camera, y_sort, zoom_camera},
        resources::{GameRegistry, Textures},
        world::{
            resources::{LoadedChunks, Settings, WorldSeed},
            systems::manage_chunks,
        },
    },
    icon::AppIconPlugin,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

#[derive(SystemSet, Debug, Clone, Hash, Eq, PartialEq)]
pub enum StartupSet {
    Assets,
    Actors,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                TILE_SIZE,
            ))
            .add_plugins(RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            })
            .add_plugins(CommandsPlugin)
            .insert_resource(ClearColor(Color::BLACK))
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
                (configure_physics, spawn_camera, spawn_player)
                    .chain()
                    .in_set(StartupSet::Actors),
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

fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}
