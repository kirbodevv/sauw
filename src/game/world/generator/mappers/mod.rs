use bevy::prelude::*;

use biome::init_biome_mapper;
use layer::init_layer_mapper;

pub mod biome;
pub mod layer;

pub use biome::BiomeMapper;
pub use layer::LayerMapper;

use crate::game::GameState;

pub struct MappersPlugin;

impl Plugin for MappersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Gaming),
            (init_layer_mapper, init_biome_mapper).chain(),
        );
    }
}
