use bevy::{input::InputSystems, prelude::*};
use virtual_joystick::VirtualJoystickMessage;

use crate::game::ui::joystick::JoystickControllerID;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

pub fn player_movement(
    mut joystick_reader: MessageReader<VirtualJoystickMessage<JoystickControllerID>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
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

    for joystick in joystick_reader.read() {
        let Vec2 { x, y } = joystick.axis();
        match joystick.id() {
            JoystickControllerID::Main => {
                dir.x += x;
                dir.y += y;
            }
        }
    }

    player_input.move_direction = dir;
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (player_movement,).after(InputSystems))
            .add_systems(PreUpdate, reset_player_input.before(InputSystems));
    }
}
