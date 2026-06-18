use bevy::prelude::*;

use crate::game::{
    crafting::Recipe,
    registry::{Registry, item_registry::ItemRegistry},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecipeId(pub u32);

#[derive(Resource)]
pub struct RecipeRegistry {
    inner: Registry<Recipe>,
}

impl RecipeRegistry {
    pub fn find_by_id(&self, id: RecipeId) -> &Recipe {
        self.inner.get(id.0 as usize).unwrap()
    }

    pub fn try_id_by_name(&self, name: &str) -> Option<RecipeId> {
        self.inner
            .ids
            .get(name)
            .copied()
            .map(|id| RecipeId(id as u32))
    }
}

pub fn init_recipes(mut commands: Commands, item_registry: Res<ItemRegistry>) {
    let mut inner = Registry::new("recipe");

    let vegetable_fiber = item_registry.id_by_name("vegetable_fiber");
    let stone = item_registry.id_by_name("stone");
    let stick = item_registry.id_by_name("stick");
    let rope = item_registry.id_by_name("rope");
    let stone_pickaxe = item_registry.id_by_name("stone_pickaxe");
    let stone_axe = item_registry.id_by_name("stone_axe");
    let stone_shovel = item_registry.id_by_name("stone_shovel");

    inner.insert(Recipe::new(rope, 1, vec![(vegetable_fiber, 3)]), "rope");
    inner.insert(
        Recipe::new(stone_pickaxe, 1, vec![(rope, 1), (stone, 3), (stick, 2)]),
        "stone_pickaxe",
    );
    inner.insert(
        Recipe::new(stone_axe, 1, vec![(rope, 1), (stone, 3), (stick, 2)]),
        "stone_axe",
    );
    inner.insert(
        Recipe::new(stone_shovel, 1, vec![(rope, 1), (stone, 1), (stick, 2)]),
        "stone_shovel",
    );

    let recipes = RecipeRegistry { inner };
    commands.insert_resource(recipes);
}
