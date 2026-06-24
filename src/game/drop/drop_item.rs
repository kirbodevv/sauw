use bevy::prelude::*;

use crate::{
    constants::TILE_SIZE,
    game::{
        drop::components::DropItem,
        player::{Player, inventory::Inventory},
        ui::hud::hotbar::SelectedHotbarSlot,
    },
};

use super::components::{DroppedBy, SpawnDrop};

pub fn drop_item(
    player_q: Single<(Entity, &mut Transform, &mut Inventory), With<Player>>,
    selected_slot: Res<SelectedHotbarSlot>,
    mut message_reader: MessageReader<DropItem>,
    mut message_writer: MessageWriter<SpawnDrop>,
) {
    let (entity, transform, mut inventory) = player_q.into_inner();
    for _ in message_reader.read() {
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
