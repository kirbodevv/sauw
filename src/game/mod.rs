#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub mod commands;
pub mod player;
pub mod registry;
pub mod world;

use crate::{
    constants::TILE_SIZE,
    game::{
        commands::CommandsPlugin,
        player::PlayerPlugin,
        registry::RegistryPlugin,
        world::{WorldPlugin, resources::Settings},
    },
    icon::AppIconPlugin,
};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Bootstrap,
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
                .continue_to_state(GameState::Bootstrap)
                .load_collection::<ImageAssets>(),
        )
        .add_plugins((RegistryPlugin, WorldPlugin, PlayerPlugin, CommandsPlugin))
        .insert_resource(Settings { load_radius: 2 })
        .insert_resource(ClearColor(Color::BLACK));
    }
}
