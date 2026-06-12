use bevy::prelude::*;

use crate::game::{ImageAssets, registry::Registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

pub struct ItemDefinition {
    pub name: &'static str,
    pub texture: Handle<Image>,
}

impl Default for ItemDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            texture: Handle::default(),
        }
    }
}

#[derive(Resource)]
pub struct ItemRegistry {
    inner: Registry<ItemDefinition>,
}

impl ItemRegistry {
    pub fn new(assets: &ImageAssets) -> Self {
        let mut inner = Registry::new("item");

        inner.insert(
            ItemDefinition {
                name: "air",
                texture: assets.block_grass.clone(),
            },
            "air",
        );

        Self { inner }
    }

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

pub fn init_items(mut commands: Commands, assets: Res<ImageAssets>) {
    let items = ItemRegistry::new(&assets);
    commands.insert_resource(items);
}
