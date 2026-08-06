use crate::game::{
    registry::biome_registry::{BiomeId, BiomeRegistry},
    world::generator::mappers::{BiomeMapper, LayerId},
};

pub fn generate(
    mapper: &BiomeMapper,
    biomes: &BiomeRegistry,
    layer: LayerId,
    temp: f64,
    humid: f64,
) -> BiomeId {
    let biome = mapper
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
        .map(|rule| rule.biome.to_string())
        .unwrap_or("desert".to_string());
    biomes.id_by_name(&biome)
}
