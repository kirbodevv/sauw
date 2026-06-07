use bevy::{input::InputSystems, prelude::*};
use virtual_joystick::VirtualJoystickMessage;

use crate::game::ui::hud::joystick::{JoystickControllerID, USE_JOYSTICK};

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

pub fn keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
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

    if dir != Vec2::ZERO {
        player_input.move_direction = dir;
    }
}

pub fn joystick_input(
    mut joystick_reader: MessageReader<VirtualJoystickMessage<JoystickControllerID>>,
    mut player_input: ResMut<PlayerInput>,
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

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_player_input.before(InputSystems))
            .add_systems(PreUpdate, (keyboard_input).after(InputSystems));

        if USE_JOYSTICK {
            app.add_systems(PreUpdate, joystick_input.after(InputSystems));
        }
    }
}
