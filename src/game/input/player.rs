use bevy::prelude::*;
use virtual_joystick::VirtualJoystickMessage;

use crate::game::{player::movement::PlayerInputState, ui::hud::joystick::JoystickControllerID};

pub fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInputState>,
) {
    let mut dir = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    player_input.move_direction = dir;
}

pub fn joystick_input(
    mut joystick_reader: MessageReader<VirtualJoystickMessage<JoystickControllerID>>,
    mut player_input: ResMut<PlayerInputState>,
) {
    let mut dir = Vec2::ZERO;

    for joystick in joystick_reader.read() {
        let Vec2 { x, y } = joystick.axis();
        match joystick.id() {
            JoystickControllerID::Main => {
                dir.x += x;
                dir.y += y;
            }
        }
    }

    if player_input.move_direction == Vec2::ZERO {
        player_input.move_direction = dir;
    }
}
