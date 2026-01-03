use bevy::prelude::*;

use crate::registry::block_registry::BlockRegistry;

#[derive(Resource)]
pub struct GameRegistry {
    pub blocks: BlockRegistry,
}

impl GameRegistry {
    pub fn new() -> Self {
        Self {
            blocks: BlockRegistry::new(),
        }
    }
}
