use bevy::prelude::*;
use bevy_console::ConsoleOpen;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    constants::TILE_SIZE,
    game::player::{
        Player,
        input::PlayerInput,
        sprite::{PlayerAnimation, PlayerState},
    },
};

pub fn player_movement(
    console_open: Res<ConsoleOpen>,
    mut query: Query<(&mut Velocity, &mut PlayerAnimation), With<Player>>,
    player_input: ResMut<PlayerInput>,
) {
    if console_open.open {
        return;
    }

    let dir = player_input.move_direction;

    for (mut velocity, mut anim) in &mut query {
        if dir != Vec2::ZERO {
            let v = dir.normalize();
            velocity.linvel = v * TILE_SIZE * 4.0;
        } else {
            velocity.linvel = Vec2::ZERO;
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

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .insert_resource(PlayerInput::default());
    }
}
