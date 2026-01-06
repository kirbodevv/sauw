use crate::game::{registry::Registry, world::block::*};

pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<&'static str>,
}

pub struct BlockRegistry {
    inner: Registry<BlockDefinition>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut inner = Registry::new();

        inner.insert(BlockDefinition {
            name: "air",
            texture: None,
        });

        inner.insert(BlockDefinition {
            name: "grass",
            texture: Some("block/grass"),
        });

        inner.insert(BlockDefinition {
            name: "tree",
            texture: Some("block/tree"),
        });

        inner.insert(BlockDefinition {
            name: "dirt",
            texture: Some("block/dirt"),
        });

        inner.insert(BlockDefinition {
            name: "flowers",
            texture: Some("block/flowers"),
        });

        inner.insert(BlockDefinition {
            name: "sand",
            texture: Some("block/sand"),
        });

        inner.insert(BlockDefinition {
            name: "stone",
            texture: Some("block/stone"),
        });

        inner.insert(BlockDefinition {
            name: "water",
            texture: Some("block/water"),
        });

        Self { inner }
    }

    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        self.inner
            .get(id.0 as usize)
            .unwrap_or_else(|| panic!("Unknown BlockId {:?}", id))
    }
}
