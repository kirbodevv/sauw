#![windows_subsystem = "windows"]
use crate::game::GamePlugin;
use bevy::prelude::*;

mod constants;
mod game;
mod icon;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
