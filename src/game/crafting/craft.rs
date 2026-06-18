use bevy::prelude::*;

use crate::game::{
    crafting::components::CraftItem,
    item::item_stack::ItemStack,
    player::{Player, inventory::Inventory},
    registry::recipe_registry::RecipeRegistry,
};

pub fn craft_item(
    mut message_reader: MessageReader<CraftItem>,
    player_q: Single<&mut Inventory, With<Player>>,
    recipe_registry: Res<RecipeRegistry>,
) {
    let mut inventory = player_q.into_inner();
    for message in message_reader.read() {
        let recipe = recipe_registry.find_by_id(message.recipe_id);
        for ingredient in &recipe.ingredients {
            if !inventory.contains(ingredient.item, ingredient.count) {
                return;
            }
        }
        for ingredient in &recipe.ingredients {
            inventory.consume(ingredient.item, ingredient.count);
        }
        inventory.add_item(ItemStack {
            item: recipe.result,
            count: recipe.count,
        });
    }
}
