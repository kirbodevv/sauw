use bevy::prelude::*;

use crate::game::{
    player::{
        input::InputPlugin, movement::MovementPlugin, spawn::PlayerSpawnPlugin,
        sprite::SpritePlugin,
    },
    world::ChunkCoord,
};

pub mod health;
pub mod input;
pub mod movement;
pub mod spawn;
pub mod sprite;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct CurrentPlayerChunk(pub Option<ChunkCoord>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerSpawnPlugin, InputPlugin, MovementPlugin, SpritePlugin))
            .insert_resource(CurrentPlayerChunk(None));
    }
}
