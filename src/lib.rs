use crate::game::GamePlugin;
#[cfg(not(target_os = "android"))]
use crate::icon::AppIconPlugin;
use bevy::prelude::*;

pub mod constants;
pub mod game;
pub mod icon;

#[bevy_main]
fn main() {
    run();
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(GamePlugin);
    #[cfg(not(target_os = "android"))]
    app.add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"));
    app.run();
}
