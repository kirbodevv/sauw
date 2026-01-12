#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub mod commands;
pub mod player;
pub mod registry;
pub mod rendering;
pub mod world;

use crate::{
    constants::TILE_SIZE,
    game::{
        commands::CommandsPlugin,
        player::PlayerPlugin,
        registry::RegistryPlugin,
        rendering::{TargetCameraZoom, camera_follow, spawn_camera, y_sort, zoom_camera},
        world::{
            resources::{LoadedChunks, Settings, WorldSeed},
            systems::manage_chunks,
        },
    },
    icon::AppIconPlugin,
};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetsLoading,
    RegistryInit,
    InitWorld,
    SpawnPlayer,
    Gaming,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AppIconPlugin::new("assets/icon/icon_128.png"),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_SIZE),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::RegistryInit)
                .load_collection::<ImageAssets>(),
        )
        .add_plugins((RegistryPlugin, PlayerPlugin, CommandsPlugin))
        .insert_resource(LoadedChunks::new())
        .insert_resource(WorldSeed(0))
        .insert_resource(TargetCameraZoom(1.0))
        .insert_resource(Settings { load_radius: 2 })
        .add_systems(
            OnEnter(GameState::InitWorld),
            (configure_physics, spawn_camera).chain(),
        )
        .add_systems(
            Update,
            (camera_follow, zoom_camera, manage_chunks, y_sort)
                .chain()
                .run_if(in_state(GameState::Gaming)),
        )
        .insert_resource(ClearColor(Color::BLACK));
    }
}

fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}
