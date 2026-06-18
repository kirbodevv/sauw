use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{ImageAssets, registry::item_registry::ItemRegistry},
};

use super::components::{Drop, DroppedBy, PickupLocked, SpawnDrop};

pub fn spawn_drop(
    mut commands: Commands,
    mut spawn_reader: MessageReader<SpawnDrop>,
    item_registry: Res<ItemRegistry>,
    assets: Res<ImageAssets>,
) {
    for event in spawn_reader.read() {
        let item = item_registry.get(event.id);
        let mut entity = commands.spawn((
            Drop {
                id: event.id,
                count: event.count,
            },
            Sprite {
                image: assets.atlas_item_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: item_registry.atlas_layout.clone(),
                    index: item.atlas_index,
                }),
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            event.dropped_by.clone(),
            Velocity::zero(),
            Transform::from_translation(event.position.map(|v| v * TILE_SIZE).extend(20.0)),
        ));

        if let DroppedBy::Player(_) = event.dropped_by {
            entity.insert(PickupLocked);
        }
    }
}
