use bevy::prelude::*;

use crate::game::{
    player::{movement::MovementPlugin, spawn::PlayerSpawnPlugin, sprite::SpritePlugin},
    world::ChunkCoord,
};

pub mod health;
pub mod inventory;
pub mod movement;
pub mod spawn;
pub mod sprite;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerLight;

#[derive(Resource)]
pub struct CurrentPlayerChunk(pub Option<ChunkCoord>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerSpawnPlugin, MovementPlugin, SpritePlugin))
            .insert_resource(CurrentPlayerChunk(None));
    }
}
