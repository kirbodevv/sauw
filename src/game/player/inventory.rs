use bevy::prelude::*;

use crate::game::{item::item_stack::ItemStack, registry::item_registry::ItemId};

pub const INVENTORY_SIZE: usize = 30;
pub const MAX_STACK_SIZE: u32 = 64;

#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }

    pub fn add_item(&mut self, mut stack: ItemStack) -> Option<ItemStack> {
        for slot in self.slots.iter_mut() {
            if let Some(existing) = slot {
                if existing.can_stack_with(&stack) {
                    let leftover = existing.add(stack.count, MAX_STACK_SIZE);
                    if leftover == 0 {
                        return None;
                    }
                    stack.count = leftover;
                }
            }
        }

        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                let to_place = stack.count.min(MAX_STACK_SIZE);
                let remaining = stack.count - to_place;
                *slot = Some(ItemStack::new(stack.item, to_place));
                if remaining == 0 {
                    return None;
                }
                stack.count = remaining;
            }
        }

        Some(stack)
    }

    pub fn take_slot(&mut self, index: usize) -> Option<ItemStack> {
        self.slots.get_mut(index)?.take()
    }

    pub fn take_from_slot(&mut self, index: usize, amount: u32) -> Option<ItemStack> {
        let slot = self.slots.get_mut(index)?.as_mut()?;
        let taken = slot.split(amount);
        if slot.is_empty() {
            self.slots[index] = None;
        }
        Some(taken)
    }

    pub fn get(&self, index: usize) -> Option<&ItemStack> {
        self.slots.get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ItemStack> {
        self.slots.get_mut(index)?.as_mut()
    }

    pub fn used_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    pub fn free_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_none()).count()
    }

    pub fn contains(&self, item_id: ItemId, amount: u32) -> bool {
        let total: u32 = self
            .slots
            .iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item == item_id)
            .map(|s| s.count)
            .sum();
        total >= amount
    }

    pub fn consume(&mut self, item_id: ItemId, mut amount: u32) -> bool {
        if !self.contains(item_id, amount) {
            return false;
        }

        for slot in self.slots.iter_mut() {
            if amount == 0 {
                break;
            }
            if let Some(stack) = slot {
                if stack.item == item_id {
                    let taken = stack.count.min(amount);
                    stack.count -= taken;
                    amount -= taken;
                }
            }
        }

        for slot in self.slots.iter_mut() {
            if let Some(stack) = slot {
                if stack.is_empty() {
                    *slot = None;
                }
            }
        }

        true
    }
}
