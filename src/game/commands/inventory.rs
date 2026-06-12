use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::{Parser, Subcommand};

use crate::game::{
    item::item_stack::ItemStack,
    player::{Player, inventory::Inventory},
    registry::item_registry::ItemRegistry,
};

#[derive(Subcommand)]
pub enum InventoryAction {
    Get,
    Add {
        item: String,
        #[arg(default_value_t = 1)]
        amount: u32,
    },
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "inventory")]
pub struct InventoryCommand {
    #[command(subcommand)]
    action: InventoryAction,
}

pub fn inventory(
    mut log: ConsoleCommand<InventoryCommand>,
    mut q_inventory: Single<&mut Inventory, With<Player>>,
    item_registry: Res<ItemRegistry>,
) {
    if let Some(Ok(InventoryCommand { action })) = log.take() {
        match action {
            InventoryAction::Get => {
                let used = q_inventory.used_slots();
                let total = q_inventory.slots.len();
                log.reply(format!("Инвентарь ({used}/{total}):"));

                let mut found_any = false;
                for (i, slot) in q_inventory.slots.iter().enumerate() {
                    if let Some(stack) = slot {
                        let def = item_registry.get(stack.item);
                        log.reply(format!("  [{i}] {} x{}", def.name, stack.count));
                        found_any = true;
                    }
                }

                if !found_any {
                    log.reply("  (пусто)");
                }
            }

            InventoryAction::Add { item, amount } => {
                let item_id = match item_registry.try_id_by_name(&item) {
                    Some(id) => id,
                    None => {
                        log.reply_failed(format!("Неизвестный предмет: \"{item}\""));
                        return;
                    }
                };

                let stack = ItemStack::new(item_id, amount);
                match q_inventory.add_item(stack) {
                    None => {
                        log.reply(format!("Добавлено {amount}x {item}"));
                    }
                    Some(leftover) => {
                        let placed = amount - leftover.count;
                        log.reply_failed(format!(
                            "Инвентарь переполнен: добавлено {placed}x {item}, не вошло {}",
                            leftover.count
                        ));
                    }
                }
            }
        }
    }
}
