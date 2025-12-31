use crate::constants::TILE_SIZE;
use bevy::prelude::*;

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile = asset_server.load("blocks/grass.png");

    for y in 0..9 {
        for x in 0..16 {
            commands.spawn((
                Sprite {
                    image: tile.clone(),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    0.0,
                ),
            ));
        }
    }
}
