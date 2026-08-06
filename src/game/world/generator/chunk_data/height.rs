use bevy_rapier2d::na::clamp;
use noise::NoiseFn;

use crate::game::world::generator::noise::{WorldNoise, normalize};

pub fn generate(noise: &WorldNoise, x: f64, y: f64) -> f64 {
    let terrain = normalize(noise.terrain.get([
        x * noise.settings.height_scale,
        y * noise.settings.height_scale,
    ]));

    let continent = normalize(noise.continent.get([x * 0.0001, y * 0.0001]));

    let continent_bias = (continent - 0.5) * 2.0;

    let height = terrain + continent_bias * 0.35;

    clamp(height, 0.0, 1.0).powf(1.3)
}
