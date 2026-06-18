use bevy::prelude::*;
use std::collections::HashMap;

use crate::game::GameState;

pub mod biome_registry;
pub mod block_registry;
pub mod item_registry;
pub mod recipe_registry;

pub struct Registry<Def> {
    type_name: &'static str,
    ids: HashMap<String, usize>,
    entries: Vec<Def>,
}

impl<Def> Registry<Def> {
    pub fn new(type_name: &'static str) -> Self {
        Self {
            type_name,
            ids: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, def: Def, name: &str) -> usize {
        let id = self.entries.len();
        self.entries.push(def);
        self.ids.insert(name.to_string(), id);
        info!(target: "registry", "Inserted {} {:?} with id {}", self.type_name, name, id);
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
            .unwrap_or_else(|| panic!("Unknown {} {:?}", self.type_name, name))
    }

    pub fn try_id_by_name(&self, name: &str) -> Option<usize> {
        self.ids.get(name).copied()
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
            (
                block_registry::init_blocks,
                item_registry::init_items,
                biome_registry::init_biomes,
                recipe_registry::init_recipes,
                next_state,
            )
                .chain(),
        );
    }
}

pub fn next_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Gaming);
}
