use noise::NoiseFn;

use crate::game::world::generator::noise::{WorldNoise, normalize};

#[derive(Default, Clone, Copy)]
pub struct CellClimate {
    pub temp: f64,
    pub humid: f64,
}

pub fn generate(noise: &WorldNoise, x: f64, y: f64) -> CellClimate {
    let temp = normalize(noise.temp.get([
        x * noise.settings.temperature_scale,
        y * noise.settings.temperature_scale,
    ]));

    let humid = normalize(noise.humid.get([
        x * noise.settings.humidity_scale,
        y * noise.settings.humidity_scale,
    ]));

    CellClimate { temp, humid }
}
