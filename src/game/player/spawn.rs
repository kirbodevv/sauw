use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    GameState, ImageAssets,
    player::{
        Player,
        health::Health,
        sprite::{PlayerAnimation, PlayerState},
    },
    ui::health::SpawnPlayerHearts,
    world::camera::YSort,
};

pub const MAX_PLAYER_HEALTH: u8 = 20;

fn spawn_player(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut ev_spawn_player_hears: MessageWriter<SpawnPlayerHearts>,
) {
    let texture = assets.entity_player.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 10, y: 26 }, 4, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let size = Vec2::new(32.0 * 10.0 / 26.0, 32.0);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            custom_size: Some(size.clone()),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0),
        Player,
        PlayerAnimation {
            previous_state: PlayerState::IdleDown,
            state: PlayerState::IdleDown,
            frame_index: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
        YSort { z: 1.0 },
        RigidBody::Dynamic,
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Health::new(MAX_PLAYER_HEALTH),
    ));

    ev_spawn_player_hears.write(SpawnPlayerHearts {
        count: MAX_PLAYER_HEALTH as usize,
    });
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
