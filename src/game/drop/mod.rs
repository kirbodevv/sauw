use bevy::prelude::*;
use bevy_rapier2d::dynamics::{RigidBody, Velocity};

use crate::{
    constants::TILE_SIZE,
    game::{
        GameState, ImageAssets,
        item::item_stack::ItemStack,
        player::{Player, inventory::Inventory},
        registry::item_registry::{ItemId, ItemRegistry},
    },
};

#[derive(Component)]
pub struct Drop {
    id: ItemId,
}

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
            Drop { id: event.id },
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
            Velocity::zero(),
            Transform::from_translation(event.position.map(|v| v * TILE_SIZE).extend(20.0)),
        ));
    }
}

pub fn collect_drop(
    mut commands: Commands,
    mut drop_q: Query<(Entity, &Transform, &Drop), Without<Player>>,
    player_q: Single<(&mut Transform, &mut Inventory), With<Player>>,
) {
    let (transform, mut inventory) = player_q.into_inner();
    let player_pos = transform.translation.xy();

    for (drop_entity, drop_transform, drop) in drop_q.iter_mut() {
        let drop_pos = drop_transform.translation.xy();
        let drop_distance = drop_pos.distance(player_pos);

        if drop_distance < 0.1 * TILE_SIZE {
            commands.entity(drop_entity).despawn();
            inventory.add_item(ItemStack {
                item: drop.id,
                count: 1,
            });
        }
    }
}

pub fn move_drop_to_player(
    mut drop_q: Query<(&mut Velocity, &Transform), (With<Drop>, Without<Player>)>,
    player_transform: Single<&mut Transform, With<Player>>,
) {
    let player_pos = player_transform.translation.xy();

    for (mut velocity, drop_transform) in drop_q.iter_mut() {
        let drop_pos = drop_transform.translation.xy();
        let drop_distance = drop_pos.distance(player_pos);

        if drop_distance <= TILE_SIZE * 1.5 {
            let dir = (player_pos - drop_pos).normalize();
            velocity.linear = dir * 1.5 * TILE_SIZE;
        } else {
            velocity.linear = Vec2::ZERO;
        }
    }
}

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDrop>().add_systems(
            Update,
            (spawn_drop, collect_drop, move_drop_to_player).run_if(in_state(GameState::Gaming)),
        );
    }
}
