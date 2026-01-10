use bevy::{input::InputSystems, prelude::*};

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

pub fn player_movement(keyboard: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
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

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (player_movement,).after(InputSystems))
            .add_systems(PreUpdate, reset_player_input.before(InputSystems));
    }
}
