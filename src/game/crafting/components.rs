use bevy::prelude::*;

use crate::game::registry::recipe_registry::RecipeId;

#[derive(Message)]
pub struct CraftItem {
    pub recipe_id: RecipeId,
}
