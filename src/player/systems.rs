use super::components::Player;
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
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

    for mut transform in &mut query {
        transform.translation += (dir.normalize_or_zero() * 200.0 * time.delta_secs()).extend(0.0);
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(300.0, 20.0, 50.0),
        Player,
    ));
}
