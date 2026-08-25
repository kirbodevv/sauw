use bevy::prelude::*;

use crate::game::{
    assets::{atlas::TextureId, resource::AtlasAssetsParam},
    registry::Registry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

pub struct ItemDefinition {
    pub name: &'static str,
    pub texture_id: TextureId,
}

impl Default for ItemDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            texture_id: TextureId(0),
        }
    }
}

#[derive(Resource)]
pub struct ItemRegistry {
    inner: Registry<ItemDefinition>,
}

impl ItemRegistry {
    #[allow(dead_code)]
    pub fn get(&self, id: ItemId) -> &ItemDefinition {
        self.inner
            .get(id.0 as usize)
            .unwrap_or_else(|| panic!("Unknown Item {:?}", id))
    }

    #[allow(dead_code)]
    pub fn by_name(&self, name: &str) -> &ItemDefinition {
        self.inner
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown Item {:?}", name))
    }

    #[allow(dead_code)]
    pub fn id_by_name(&self, name: &str) -> ItemId {
        ItemId(self.inner.id_by_name(name) as u16)
    }

    pub fn try_id_by_name(&self, name: &str) -> Option<ItemId> {
        self.inner.try_id_by_name(name).map(|id| ItemId(id as u16))
    }
}

pub fn init_items(mut commands: Commands, atlas_assets: AtlasAssetsParam) {
    let atlas = atlas_assets.item_atlas();
    let mut inner = Registry::new("item");

    let mut insert_item = |name: &'static str| {
        let texture_id = atlas
            .try_id_by_name(name)
            .unwrap_or_else(|| panic!("Item {:?} has no atlas entry", name));

        inner.insert(ItemDefinition { name, texture_id }, name)
    };

    insert_item("aluminium_can");
    insert_item("apple");
    insert_item("furnace");
    insert_item("hammer");
    insert_item("handsaw");
    insert_item("iron_ingot");
    insert_item("iron_ore");
    insert_item("iron_plate");
    insert_item("log");
    insert_item("peanut");
    insert_item("planks");
    insert_item("rope");
    insert_item("sapling");
    insert_item("stick");
    insert_item("stick_1");
    insert_item("stone");
    insert_item("stone_axe");
    insert_item("stone_pickaxe");
    insert_item("stone_shovel");
    insert_item("vegetable_fiber");
    insert_item("watermelon");

    commands.insert_resource(ItemRegistry { inner });
}
