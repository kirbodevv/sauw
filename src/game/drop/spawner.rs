use bevy::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{
        GameState, ImageAssets,
        registry::item_registry::{ItemId, ItemRegistry},
    },
};

#[derive(Message)]
pub struct SpawnDrop {
    pub id: ItemId,
    pub position: Vec2,
}

pub fn spawn_drop(
    mut commands: Commands,
    mut spawn_reader: MessageReader<SpawnDrop>,
    item_registry: Res<ItemRegistry>,
    assets: Res<ImageAssets>,
) {
    for event in spawn_reader.read() {
        let item = item_registry.get(event.id);

        commands.spawn((
            Sprite {
                image: assets.atlas_item_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: item_registry.atlas_layout.clone(),
                    index: item.atlas_index,
                }),
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
            Transform::from_translation(event.position.map(|v| v * TILE_SIZE).extend(20.0)),
        ));
    }
}

pub struct DropSpawnerPlugin;

impl Plugin for DropSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDrop>()
            .add_systems(Update, spawn_drop.run_if(in_state(GameState::Gaming)));
    }
}
