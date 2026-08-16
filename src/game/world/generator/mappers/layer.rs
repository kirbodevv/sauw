use crate::game::assets::{resource::WorldgenMapperAssets, worldgen::LayerMapperAsset};
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LayerId(pub u8);

#[derive(Resource)]
pub struct LayerMapper {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub name: String,
    pub height: (f64, f64),
}

impl LayerMapper {
    pub fn id_by_name(&self, name: &str) -> LayerId {
        self.layers
            .iter()
            .position(|layer| layer.name.as_str() == name)
            .map(|i| LayerId(i as u8))
            .unwrap()
    }
}

pub fn init_layer_mapper(
    mut commands: Commands,
    mapper: Res<Assets<LayerMapperAsset>>,
    assets: Res<WorldgenMapperAssets>,
) {
    let handle = &assets.layer_mapper;

    let Some(map) = mapper.get(handle) else {
        return;
    };

    let layers = map
        .layers
        .iter()
        .map(|layer| Layer {
            name: layer.name.clone(),
            height: (layer.height[0], layer.height[1]),
        })
        .collect::<Vec<_>>();

    let layer_mapper = LayerMapper { layers };

    commands.insert_resource(layer_mapper);
}
