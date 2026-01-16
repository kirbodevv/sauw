#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

use crate::{
    constants::TILE_SIZE,
    game::{
        commands::CommandsPlugin, player::PlayerPlugin, registry::RegistryPlugin, ui::UiPlugin,
        world::WorldPlugin,
    },
    icon::AppIconPlugin,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod commands;
pub mod player;
pub mod registry;
pub mod ui;
pub mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Bootstrap,
    Gaming,
    GameOver,
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
        .add_plugins((
            RegistryPlugin,
            WorldPlugin,
            PlayerPlugin,
            CommandsPlugin,
            UiPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK));
    }
}
