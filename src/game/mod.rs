#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

use crate::{
    constants::TILE_SIZE,
    game::{
        assets::GameAssetsPlugin, commands::CommandsPlugin, player::PlayerPlugin,
        registry::RegistryPlugin, ui::UiPlugin, world::WorldPlugin,
    },
};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    text::FontSmoothing,
    window::{PresentMode, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_firefly::app::FireflyPlugin;
use bevy_rapier2d::prelude::*;

pub mod assets;
pub mod commands;
pub mod item;
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
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        recognize_rotation_gesture: true,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::LinearRgba(LinearRgba::GREEN),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: false,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: false,
                        min_fps: 30.0,
                        target_fps: 144.0,
                    },
                },
            },
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_SIZE),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .init_state::<GameState>()
        .add_plugins(GameAssetsPlugin)
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

        let lighting = !std::env::args().any(|a| a == "--lighting=off");

        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            use bevy_winit::WinitSettings;
            app.insert_resource(WinitSettings::mobile());
        }

        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            if lighting {
                app.add_plugins(FireflyPlugin);
            }
        }
    }
}
