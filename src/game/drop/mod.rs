use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{
        GameState, ImageAssets,
        item::item_stack::ItemStack,
        player::{Player, inventory::Inventory},
        registry::item_registry::{ItemId, ItemRegistry},
        ui::hud::hotbar::SelectedHotbarSlot,
    },
};

#[derive(Component)]
pub struct Drop {
    id: ItemId,
    count: u32,
}

#[derive(Component, Clone)]
pub enum DroppedBy {
    Player { player_moved: bool },
    Spawned,
    Command,
}

impl DroppedBy {
    pub fn is_collectable(&self) -> bool {
        matches!(
            self,
            DroppedBy::Player { player_moved: true } | DroppedBy::Spawned | DroppedBy::Command
        )
    }
}

#[derive(Message)]
pub struct SpawnDrop {
    pub id: ItemId,
    pub count: u32,
    pub position: Vec2,
    pub dropped_by: DroppedBy,
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
    }
}

pub fn drop_collection(
    mut commands: Commands,
    mut drop_q: Query<(Entity, &mut Velocity, &Transform, &Drop, &mut DroppedBy), Without<Player>>,
    player_q: Single<(&mut Transform, &mut Inventory), With<Player>>,
) {
    let (transform, mut inventory) = player_q.into_inner();
    let player_pos = transform.translation.xy();

    for (drop_entity, mut velocity, drop_transform, drop, dropped_by) in drop_q.iter_mut() {
        let drop_pos = drop_transform.translation.xy();
        let drop_distance = drop_pos.distance(player_pos);

        let is_player_far = drop_distance > 2.0 * TILE_SIZE;
        let is_near_to_move = drop_distance < 1.5 * TILE_SIZE;
        let is_near_to_collect = drop_distance < 0.1 * TILE_SIZE;
        let is_collectable = dropped_by.is_collectable();

        if is_near_to_collect && is_collectable {
            commands.entity(drop_entity).despawn();
            inventory.add_item(ItemStack {
                item: drop.id,
                count: drop.count,
            });
            return;
        }

        if let DroppedBy::Player { player_moved } = dropped_by.into_inner() {
            if !*player_moved && is_player_far {
                *player_moved = true;
            }
        }

        if is_near_to_move && is_collectable {
            let dir = (player_pos - drop_pos).normalize();
            velocity.linear = dir * 1.5 * TILE_SIZE;
        } else {
            velocity.linear = Vec2::ZERO;
        }
    }
}

pub fn drop_item(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Single<(&mut Transform, &mut Inventory), With<Player>>,
    mut message_writer: MessageWriter<SpawnDrop>,
    selected_slot: Res<SelectedHotbarSlot>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        let (transform, mut inventory) = player_q.into_inner();

        let slot = selected_slot.0;
        if let Some(item) = inventory.take_from_slot(slot, 1) {
            message_writer.write(SpawnDrop {
                id: item.item,
                count: item.count,
                position: transform.translation.xy().map(|v| v / TILE_SIZE),
                dropped_by: DroppedBy::Player {
                    player_moved: false,
                },
            });
        }
    }
}

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDrop>().add_systems(
            Update,
            (spawn_drop, drop_collection, drop_item).run_if(in_state(GameState::Gaming)),
        );
    }
}
