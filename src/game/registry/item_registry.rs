use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{ImageAssets, assets::atlas::Atlas, registry::Registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

pub struct ItemDefinition {
    pub name: &'static str,
    pub atlas_index: usize,
}

impl Default for ItemDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            atlas_index: 0,
        }
    }
}

#[derive(Resource)]
pub struct ItemRegistry {
    inner: Registry<ItemDefinition>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
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

pub fn init_items(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    atlas_assets: Res<Assets<Atlas>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas = atlas_assets
        .get(&assets.atlas_item)
        .expect("Item atlas not loaded");

    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(atlas.width, atlas.height));
    let mut name_to_index: HashMap<&str, usize> = HashMap::new();

    for (name, entry) in &atlas.entries {
        let [x, y, w, h] = [entry.x(), entry.y(), entry.width(), entry.height()];
        let idx = layout.add_texture(URect::new(x, y, x + w, y + h));
        name_to_index.insert(name.get_name(), idx);
    }

    let atlas_layout = layouts.add(layout);

    let mut inner = Registry::new("item");

    let mut insert_item = |name: &'static str| {
        inner.insert(
            ItemDefinition {
                name,
                atlas_index: name_to_index.get(name).copied().unwrap_or(0),
            },
            name,
        )
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

    commands.insert_resource(ItemRegistry {
        inner,
        atlas_layout,
    });
}
