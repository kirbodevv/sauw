use crate::game::{registry::Registry, world::block::*};

pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<&'static str>,
}

pub struct BlockRegistry {
    inner: Registry<BlockId, BlockDefinition>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut inner = Registry::new();

        inner.insert(
            BlockId(0),
            BlockDefinition {
                name: "air",
                texture: None,
            },
        );

        inner.insert(
            BlockId(1),
            BlockDefinition {
                name: "grass",
                texture: Some("block/grass"),
            },
        );

        inner.insert(
            BlockId(2),
            BlockDefinition {
                name: "tree",
                texture: Some("block/tree"),
            },
        );

        Self { inner }
    }

    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        self.inner
            .get(id)
            .unwrap_or_else(|| panic!("Unknown BlockId {:?}", id))
    }
}
