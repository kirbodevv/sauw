use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::{crafting::components::CraftItem, registry::recipe_registry::RecipeRegistry};

#[derive(Parser, ConsoleCommand)]
#[command(name = "craft")]
pub struct CraftCommand {
    pub recipe: String,
}

pub fn craft(
    mut log: ConsoleCommand<CraftCommand>,
    mut event_writer: MessageWriter<CraftItem>,
    recipe_registry: Option<Res<RecipeRegistry>>,
) {
    if let Some(Ok(CraftCommand { recipe })) = log.take()
        && let Some(registry) = recipe_registry
    {
        let id = registry.try_id_by_name(&recipe);

        let Some(recipe_id) = id else {
            log.reply_failed(format!("recipe not found: {}", recipe));
            return;
        };

        event_writer.write(CraftItem { recipe_id });
    }
}
