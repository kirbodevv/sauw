use bevy::prelude::*;

use crate::game::{
    assets::recipe::Recipe as RecipeAsset,
    crafting::{Ingredient, Recipe},
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

pub fn init_recipes(
    mut commands: Commands,
    recipes: Res<Assets<RecipeAsset>>,
    item_registry: Res<ItemRegistry>,
) {
    let mut inner = Registry::new("recipe");

    for (_id, recipe) in recipes.iter() {
        inner.insert(
            Recipe {
                result: item_registry.id_by_name(&recipe.result),
                count: recipe.count,
                ingredients: recipe
                    .ingredients
                    .iter()
                    .map(|ingredient| Ingredient {
                        item: item_registry.id_by_name(&ingredient.item),
                        count: ingredient.count as u32,
                    })
                    .collect(),
            },
            &recipe.name,
        );
    }

    let recipes = RecipeRegistry { inner };
    commands.insert_resource(recipes);
}
