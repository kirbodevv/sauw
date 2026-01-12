use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::{
    constants::CHUNK_WORLD,
    game::{
        GameState,
        player::{CurrentPlayerChunk, Player},
        world::{
            camera::CameraPlugin,
            generator::{ChunkGenerateRequest, GeneratorPlugin},
        },
    },
};

pub mod camera;
pub mod generator;

#[derive(Component)]
pub struct BlockEntity;

#[derive(Component, Clone)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub layer: u8,
}

impl BlockPos {
    pub fn new(x: u8, y: u8, layer: u8) -> Self {
        Self { x, y, layer }
    }
}

#[derive(Component)]
pub struct Chunk;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource)]
pub struct WorldSeed(pub u32);

#[derive(Resource)]
pub struct LoadedChunks {
    pub set: HashSet<ChunkCoord>,
}

#[derive(Resource)]
pub struct Settings {
    pub load_radius: i32,
}

impl LoadedChunks {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
}

fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.single_mut() else {
        return;
    };
    rapier_config.gravity = Vec2::ZERO;
}

fn manage_chunks(
    mut commands: Commands,
    mut writer: MessageWriter<ChunkGenerateRequest>,
    mut loaded: ResMut<LoadedChunks>,
    mut last_player_chunk: ResMut<CurrentPlayerChunk>,
    settings: Res<Settings>,
    player: Single<&Transform, With<Player>>,
    chunks: Query<(Entity, &ChunkCoord), With<Chunk>>,
) {
    let player_pos = player.translation;
    let current_player_chunk = ChunkCoord {
        x: (player_pos.x / CHUNK_WORLD).floor() as i32,
        y: (player_pos.y / CHUNK_WORLD).floor() as i32,
    };

    if let Some(chunk) = last_player_chunk.0 {
        if current_player_chunk == chunk {
            return;
        }
    }

    last_player_chunk.0 = Some(current_player_chunk);

    let mut required = HashSet::new();

    let load_radius = settings.load_radius;

    for cx in (current_player_chunk.x - load_radius)..=(current_player_chunk.x + load_radius) {
        for cy in (current_player_chunk.y - load_radius)..=(current_player_chunk.y + load_radius) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }

    for coord in required.iter() {
        if !loaded.set.contains(coord) {
            writer.write(ChunkGenerateRequest(*coord));
            loaded.set.insert(*coord);
        }
    }

    for (entity, chunk) in &chunks {
        if !required.contains(chunk) {
            commands.entity(entity).despawn();
        }
    }

    loaded.set = required;
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks::new())
            .insert_resource(WorldSeed(0))
            .insert_resource(Settings { load_radius: 2 })
            .add_systems(Startup, configure_physics)
            .add_systems(Update, manage_chunks.run_if(in_state(GameState::Gaming)))
            .add_plugins((CameraPlugin, GeneratorPlugin));
    }
}
