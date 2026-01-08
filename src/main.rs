use crate::game::plugins::{AppIconPlugin, GameCommandsPlugin, GamePlugin};
use avian2d::PhysicsPlugins;
use bevy::prelude::*;

mod constants;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default().with_length_unit(32.0))
        .add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"))
        .add_plugins(GamePlugin)
        .add_plugins(GameCommandsPlugin)
        .run();
}
