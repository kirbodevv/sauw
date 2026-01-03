use crate::{core::registry::Registry, world::block::*};

pub struct BlockDefinition {
    pub name: &'static str,
    pub solid: bool,
    pub behavior: BlockBehavior,
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
                solid: false,
                behavior: BlockBehavior::None,
                texture: None,
            },
        );

        inner.insert(
            BlockId(1),
            BlockDefinition {
                name: "grass",
                solid: false,
                behavior: BlockBehavior::None,
                texture: Some("block/grass"),
            },
        );

        inner.insert(
            BlockId(2),
            BlockDefinition {
                name: "tree",
                solid: true,
                behavior: BlockBehavior::Solid,
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
