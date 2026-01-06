use bevy::math::Vec2;

use crate::game::{registry::Registry, world::block::*};

#[derive(PartialEq)]
pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<&'static str>,
    pub custom_size: Option<Vec2>,
}

pub struct BlockRegistry {
    inner: Registry<BlockDefinition>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut inner = Registry::new();

        inner.insert(
            BlockDefinition {
                name: "air",
                texture: None,
                custom_size: None,
            },
            "air",
        );

        inner.insert(
            BlockDefinition {
                name: "grass",
                texture: Some("block/grass"),
                custom_size: None,
            },
            "grass",
        );

        inner.insert(
            BlockDefinition {
                name: "tree",
                texture: Some("block/tree"),
                custom_size: Some(Vec2::new(32., 64.)),
            },
            "tree",
        );

        inner.insert(
            BlockDefinition {
                name: "dirt",
                texture: Some("block/dirt"),
                custom_size: None,
            },
            "dirt",
        );

        inner.insert(
            BlockDefinition {
                name: "flowers",
                texture: Some("block/flowers"),
                custom_size: None,
            },
            "flowers",
        );

        inner.insert(
            BlockDefinition {
                name: "sand",
                texture: Some("block/sand"),
                custom_size: None,
            },
            "sand",
        );

        inner.insert(
            BlockDefinition {
                name: "stone",
                texture: Some("block/stone"),
                custom_size: None,
            },
            "stone",
        );

        inner.insert(
            BlockDefinition {
                name: "water",
                texture: Some("block/water"),
                custom_size: None,
            },
            "water",
        );

        Self { inner }
    }

    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        self.inner
            .get(id.0 as usize)
            .unwrap_or_else(|| panic!("Unknown BlockId {:?}", id))
    }

    pub fn by_name(&self, name: &str) -> &BlockDefinition {
        self.inner
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown Block {:?}", name))
    }

    pub fn id_by_name(&self, name: &str) -> BlockId {
        BlockId(self.inner.id_by_name(name) as u16)
    }
}
