use crate::game::registry::item_registry::ItemId;

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: ItemId, count: u32) -> Self {
        Self { item, count }
    }

    pub fn empty(item: ItemId) -> Self {
        Self { item, count: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self, max_stack: u32) -> bool {
        self.count >= max_stack
    }

    pub fn space_left(&self, max_stack: u32) -> u32 {
        max_stack.saturating_sub(self.count)
    }

    pub fn can_stack_with(&self, other: &ItemStack) -> bool {
        self.item == other.item
    }

    pub fn add(&mut self, amount: u32, max_stack: u32) -> u32 {
        let space = self.space_left(max_stack);
        let added = space.min(amount);

        self.count += added;
        amount - added
    }

    pub fn split(&mut self, amount: u32) -> Self {
        let taken = self.count.min(amount);
        self.count -= taken;

        Self {
            item: self.item,
            count: taken,
        }
    }

    pub fn merge(&mut self, other: &mut ItemStack, max_stack: u32) -> bool {
        if self.item != other.item {
            return false;
        }

        let space = self.space_left(max_stack);
        let moved = space.min(other.count);

        self.count += moved;
        other.count -= moved;

        true
    }
}
