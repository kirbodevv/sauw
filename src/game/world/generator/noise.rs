use bevy::prelude::*;
use noise::{Fbm, Perlin};

use crate::game::world::WorldSeed;

#[derive(Resource)]
pub struct WorldNoise {
    pub terrain: Fbm<Perlin>,
    pub continent: Fbm<Perlin>,
    pub temp: Perlin,
    pub humid: Perlin,
}

impl WorldNoise {
    pub fn new(seed: u32) -> Self {
        Self {
            terrain: Fbm::<Perlin>::new(seed),
            continent: Fbm::<Perlin>::new(seed + 9999),
            temp: Perlin::new(seed),
            humid: Perlin::new(seed + 1337),
        }
    }
}

pub fn init_noise(mut commands: Commands, seed: Res<WorldSeed>) {
    commands.insert_resource(WorldNoise::new(seed.0));
}

#[inline]
pub fn normalize(v: f64) -> f64 {
    (v + 1.0) / 2.0
}
