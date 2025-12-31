// plugins/game.rs
use bevy::prelude::*;

use crate::{
    core::{
        camera::{camera_follow, spawn_camera},
        window_icon::{IconSet, set_window_icon_once},
    },
    player::systems::{player_movement, spawn_player},
    world::tiles::spawn_tiles,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IconSet::default())
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, (spawn_camera, spawn_tiles, spawn_player))
            .add_systems(
                Update,
                (set_window_icon_once, player_movement, camera_follow).chain(),
            );
    }
}
