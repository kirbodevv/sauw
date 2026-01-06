use crate::game::{
    player::components::{PlayerAnimation, PlayerState},
    resources::Textures,
};

use super::components::Player;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    textures: Res<Textures>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = textures.entities.get("entity/player").unwrap();
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 10, y: 26 }, 4, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2::new(32.0 * 10.0 / 26.0, 32.0)),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0),
        Player,
        PlayerAnimation {
            state: PlayerState::IdleDown,
            frame_index: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
    ));
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerAnimation), With<Player>>,
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

    for (mut transform, mut anim) in &mut query {
        if dir != Vec2::ZERO {
            let v = dir.normalize() * 200.0 * time.delta_secs();
            transform.translation += v.extend(0.0);
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

pub fn player_animate(time: Res<Time>, mut query: Query<(&mut PlayerAnimation, &mut Sprite)>) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            let frames = get_frames_for_state(&anim.state);
            anim.frame_index = (anim.frame_index + 1) % frames.len();
            atlas.index = frames[anim.frame_index];
        }
    }
}

fn get_frames_for_state(state: &PlayerState) -> &'static [usize] {
    match state {
        PlayerState::IdleDown => &[8],
        PlayerState::IdleUp => &[9],
        PlayerState::IdleLeft => &[1],
        PlayerState::IdleRight => &[3],
        PlayerState::WalkDown => &[4, 5],
        PlayerState::WalkUp => &[6, 7],
        PlayerState::WalkLeft => &[0, 1],
        PlayerState::WalkRight => &[2, 3],
    }
}
