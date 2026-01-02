use bevy::prelude::*;

mod constants;
mod core;
mod player;
mod plugins;
mod startup;
mod world;

use plugins::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .run();
}
