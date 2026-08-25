use crate::game::{
    registry::biome_registry::BiomeId,
    world::generator::{
        chunk_data::climate::CellClimate,
        mappers::{BiomeMapper, layer::LayerId},
    },
};

pub fn generate(mapper: &BiomeMapper, layer: LayerId, climate: CellClimate) -> BiomeId {
    mapper
        .rules
        .iter()
        .filter(|rule| {
            if rule.layer != layer {
                return false;
            }

            if !rule
                .temp
                .is_none_or(|t| climate.temp >= t.0 && climate.temp <= t.1)
            {
                return false;
            }
            rule.humid
                .is_none_or(|h| climate.humid >= h.0 && climate.humid <= h.1)
        })
        .max_by_key(|r| r.priority)
        .map(|rule| rule.biome)
        .unwrap()
}
