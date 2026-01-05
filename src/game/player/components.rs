use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAnimation {
    pub state: PlayerState,
    pub frame_index: usize,
    pub timer: Timer,
}

#[derive(Copy, Clone)]
pub enum PlayerState {
    IdleDown,
    IdleUp,
    IdleLeft,
    IdleRight,
    WalkDown,
    WalkUp,
    WalkLeft,
    WalkRight,
}
