use bevy::prelude::*;

use crate::game::player::components::{PlayerAnimation, PlayerState};

pub fn player_animate(time: Res<Time>, mut query: Query<(&mut PlayerAnimation, &mut Sprite)>) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if (anim.timer.just_finished() || anim.state != anim.previous_state)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            anim.previous_state = anim.state;
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

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_animate);
    }
}
