use bevy::prelude::*;
use bevy_console::ConsoleOpen;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    constants::TILE_SIZE,
    game::{
        GameState,
        player::{
            CurrentPlayerChunk, Player,
            sprite::{PlayerAnimation, PlayerState},
        },
        world::ChunkCoord,
    },
};

#[derive(Resource, Default)]
pub struct PlayerInputState {
    pub move_direction: Vec2,
}

pub fn player_movement(
    console_open: Option<Res<ConsoleOpen>>,
    mut query: Query<(&mut Velocity, &mut PlayerAnimation), With<Player>>,
    player_input: ResMut<PlayerInputState>,
) {
    if let Some(console_open) = console_open
        && console_open.open
    {
        return;
    }

    let dir = player_input.move_direction;

    for (mut velocity, mut anim) in &mut query {
        if dir != Vec2::ZERO {
            let v = dir.normalize();
            velocity.linear = v * TILE_SIZE * 4.0;
        } else {
            velocity.linear = Vec2::ZERO;
        }

        anim.state = if dir == Vec2::ZERO {
            match anim.state {
                PlayerState::WalkUp => PlayerState::IdleUp,
                PlayerState::WalkDown => PlayerState::IdleDown,
                PlayerState::WalkLeft => PlayerState::IdleLeft,
                PlayerState::WalkRight => PlayerState::IdleRight,
                idle => idle,
            }
        } else {
            if dir.y.abs() > dir.x.abs() {
                if dir.y > 0.0 {
                    PlayerState::WalkUp
                } else {
                    PlayerState::WalkDown
                }
            } else {
                if dir.x > 0.0 {
                    PlayerState::WalkRight
                } else {
                    PlayerState::WalkLeft
                }
            }
        };
    }
}

pub fn update_player_chunk(
    mut chunk: ResMut<CurrentPlayerChunk>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let Vec3 { x, y, .. } = player_transform.translation;
    chunk.0 = Some(ChunkCoord::from_world_pos(x, y));
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, update_player_chunk).run_if(in_state(GameState::Gaming)),
        )
        .insert_resource(PlayerInputState::default());
    }
}
