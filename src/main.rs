use bevy::prelude::*;

mod constants;
mod core;
mod game_registry;
mod player;
mod plugins;
mod registry;
mod startup;
mod world;

use plugins::game::GamePlugin;

use crate::plugins::icon::AppIconPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"))
        .add_plugins(GamePlugin)
        .run();
}
