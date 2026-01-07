use bevy::{math::Vec2, utils::default};

use crate::game::{registry::Registry, world::block::*};

#[derive(PartialEq)]
pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<&'static str>,
    pub custom_size: Option<Vec2>,
    pub y_sort: f32,
}

impl Default for BlockDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            texture: None,
            custom_size: None,
            y_sort: 1.0,
        }
    }
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
                ..default()
            },
            "air",
        );

        inner.insert(
            BlockDefinition {
                name: "grass",
                texture: Some("block/grass"),
                ..default()
            },
            "grass",
        );

        inner.insert(
            BlockDefinition {
                name: "tree",
                texture: Some("block/tree"),
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            "tree",
        );

        inner.insert(
            BlockDefinition {
                name: "dirt",
                texture: Some("block/dirt"),
                ..default()
            },
            "dirt",
        );

        inner.insert(
            BlockDefinition {
                name: "flowers",
                texture: Some("block/flowers"),
                y_sort: 0.0,
                ..default()
            },
            "flowers",
        );

        inner.insert(
            BlockDefinition {
                name: "sand",
                texture: Some("block/sand"),
                ..default()
            },
            "sand",
        );

        inner.insert(
            BlockDefinition {
                name: "stone",
                texture: Some("block/stone"),
                ..default()
            },
            "stone",
        );

        inner.insert(
            BlockDefinition {
                name: "water",
                texture: Some("block/water"),
                ..default()
            },
            "water",
        );

        Self { inner }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn id_by_name(&self, name: &str) -> BlockId {
        BlockId(self.inner.id_by_name(name) as u16)
    }
}
