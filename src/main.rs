use bevy::prelude::*;

use crate::engine::plugins::{AppIconPlugin, GamePlugin};

mod constants;
mod engine;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"))
        .add_plugins(GamePlugin)
        .run();
}
