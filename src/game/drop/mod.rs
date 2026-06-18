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

#[derive(Component, Clone, PartialEq, Eq)]
pub enum DroppedBy {
    Player(Entity),
    Spawned,
    Command,
}

#[derive(Component)]
pub struct PickupLocked;

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

pub fn collect_drops(
    mut commands: Commands,
    mut inventory: Single<&mut Inventory, With<Player>>,
    player_q: Single<&Transform, With<Player>>,
    drops: Query<(Entity, &Transform, &Drop), Without<PickupLocked>>,
) {
    let player_pos = player_q.translation.xy();

    for (entity, transform, drop) in &drops {
        let distance = transform.translation.xy().distance(player_pos);

        if distance < 0.1 * TILE_SIZE {
            inventory.add_item(ItemStack {
                item: drop.id,
                count: drop.count,
            });

            commands.entity(entity).despawn();
        }
    }
}

pub fn move_drops(
    player_q: Single<&Transform, With<Player>>,
    mut drops: Query<(&Transform, &mut Velocity), (Without<PickupLocked>, With<Drop>)>,
) {
    let player_pos = player_q.translation.xy();

    for (transform, mut velocity) in &mut drops {
        let drop_pos = transform.translation.xy();
        let distance = drop_pos.distance(player_pos);

        if distance < 1.5 * TILE_SIZE {
            velocity.linear = (player_pos - drop_pos).normalize() * 1.5 * TILE_SIZE;
        } else {
            velocity.linear = Vec2::ZERO;
        }
    }
}

pub fn drop_item(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Single<(Entity, &mut Transform, &mut Inventory), With<Player>>,
    mut message_writer: MessageWriter<SpawnDrop>,
    selected_slot: Res<SelectedHotbarSlot>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        let (entity, transform, mut inventory) = player_q.into_inner();

        let slot = selected_slot.0;
        if let Some(item) = inventory.take_from_slot(slot, 1) {
            message_writer.write(SpawnDrop {
                id: item.item,
                count: item.count,
                position: transform.translation.xy().map(|v| v / TILE_SIZE),
                dropped_by: DroppedBy::Player(entity),
            });
        }
    }
}

pub fn unlock_drops(
    mut commands: Commands,
    player_q: Single<&Transform, With<Player>>,
    drops: Query<(Entity, &Transform), With<PickupLocked>>,
) {
    let player_pos = player_q.translation.xy();

    for (entity, transform) in &drops {
        let distance = transform.translation.xy().distance(player_pos);

        if distance > 2.0 * TILE_SIZE {
            commands.entity(entity).remove::<PickupLocked>();
        }
    }
}

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDrop>().add_systems(
            Update,
            (
                spawn_drop,
                collect_drops,
                move_drops,
                drop_item,
                unlock_drops,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
