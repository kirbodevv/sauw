use bevy::prelude::*;

use crate::game::{GameState, crafting::craft::craft_item, registry::item_registry::ItemId};

pub mod components;
pub mod craft;

pub struct Recipe {
    pub result: ItemId,
    pub count: u32,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(result: ItemId, count: u32, ingredients: Vec<(ItemId, u32)>) -> Self {
        Self {
            result,
            count,
            ingredients: ingredients
                .into_iter()
                .map(|(item, count)| Ingredient::new(item, count))
                .collect(),
        }
    }
}

pub struct Ingredient {
    pub item: ItemId,
    pub count: u32,
}

impl Ingredient {
    pub fn new(item: ItemId, count: u32) -> Self {
        Self { item, count }
    }
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<components::CraftItem>()
            .add_systems(Update, craft_item.run_if(in_state(GameState::Gaming)));
    }
}
