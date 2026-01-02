use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::world::block::*;

pub struct BlockDefinition {
    pub name: &'static str,
    pub solid: bool,
    pub behavior: BlockBehavior,
    pub texture: Option<&'static str>,
}

#[derive(Resource)]
pub struct BlockRegistry {
    blocks: HashMap<BlockId, BlockDefinition>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut blocks = HashMap::new();

        blocks.insert(
            BlockId(0),
            BlockDefinition {
                name: "air",
                solid: false,
                behavior: BlockBehavior::None,
                texture: None,
            },
        );

        blocks.insert(
            BlockId(1),
            BlockDefinition {
                name: "grass",
                solid: false,
                behavior: BlockBehavior::None,
                texture: Some("block/grass"),
            },
        );

        blocks.insert(
            BlockId(2),
            BlockDefinition {
                name: "tree",
                solid: true,
                behavior: BlockBehavior::Solid,
                texture: Some("block/tree"),
            },
        );

        Self { blocks }
    }

    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        &self.blocks[&id]
    }
}
