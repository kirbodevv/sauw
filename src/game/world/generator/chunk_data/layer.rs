use crate::game::world::generator::mappers::{LayerId, LayerMapper};

pub fn generate(mapper: &LayerMapper, height: f64) -> LayerId {
    LayerId(
        mapper
            .layers
            .iter()
            .position(|layer| height >= layer.height.0 && height <= layer.height.1)
            .unwrap() as u8,
    )
}
