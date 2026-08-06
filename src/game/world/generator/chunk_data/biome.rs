use crate::game::{
    registry::biome_registry::BiomeId,
    world::generator::mappers::{BiomeMapper, LayerId},
};

pub fn generate(mapper: &BiomeMapper, layer: LayerId, temp: f64, humid: f64) -> BiomeId {
    mapper
        .rules
        .iter()
        .filter(|rule| {
            if rule.layer != layer {
                return false;
            }

            if !rule.temp.is_none_or(|t| temp >= t.0 && temp <= t.1) {
                return false;
            }
            rule.humid.is_none_or(|h| humid >= h.0 && humid <= h.1)
        })
        .max_by_key(|r| r.priority)
        .map(|rule| rule.biome)
        .unwrap()
}
