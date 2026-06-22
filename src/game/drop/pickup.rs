use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{
        item::item_stack::ItemStack,
        player::{Player, inventory::Inventory},
    },
};

use super::components::{Drop, PickupLocked};

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

#[allow(clippy::type_complexity)]
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
