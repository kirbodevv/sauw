use bevy::prelude::*;

use crate::game::{
    drop::components::DropItem,
    ui::hud::hotbar::{ChangeSelectedHotbarSlot, HOTBAR_SLOT_COUNT},
};

const DIGIT_KEYS: [KeyCode; HOTBAR_SLOT_COUNT] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
];

pub fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut change_slot_writer: MessageWriter<ChangeSelectedHotbarSlot>,
    mut drop_item_writer: MessageWriter<DropItem>,
) {
    let new_index = DIGIT_KEYS
        .iter()
        .enumerate()
        .find_map(|(i, key)| input.just_pressed(*key).then_some(i));

    if let Some(index) = new_index {
        change_slot_writer.write(ChangeSelectedHotbarSlot(index));
    }

    if input.just_pressed(KeyCode::KeyQ) {
        drop_item_writer.write(DropItem);
    }
}
