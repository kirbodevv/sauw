use bevy::prelude::*;
use std::collections::HashMap;

use crate::game::{
    GameState,
    registry::{biome_registry::init_biomes, block_registry::init_blocks},
};

pub mod biome_registry;
pub mod block_registry;

pub struct Registry<Def> {
    ids: HashMap<String, usize>,
    entries: Vec<Def>,
}

impl<Def> Registry<Def> {
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, def: Def, name: &str) -> usize {
        let id = self.entries.len();
        self.entries.push(def);
        self.ids.insert(name.to_string(), id);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Def> {
        self.entries.get(id)
    }

    pub fn by_name(&self, name: &str) -> Option<&Def> {
        self.ids.get(name).and_then(|&id| self.entries.get(id))
    }

    pub fn id_by_name(&self, name: &str) -> usize {
        self.ids
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("Unknown Block {:?}", name))
    }

    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        self.entries.get(id).is_some()
    }
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Bootstrap),
            (init_blocks, init_biomes, next_state).chain(),
        );
    }
}

pub fn next_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Gaming);
}
