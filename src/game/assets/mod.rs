pub mod atlas;
pub mod worldgen;

use crate::game::{
    assets::atlas::{Atlas, AtlasLoader},
    assets::worldgen::{Biome, BiomeLoader, BiomeMapper, BiomeMapperLoader},
};
use bevy::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Atlas>()
            .init_asset_loader::<AtlasLoader>()
            .init_asset::<BiomeMapper>()
            .init_asset_loader::<BiomeMapperLoader>()
            .init_asset::<Biome>()
            .init_asset_loader::<BiomeLoader>();
    }
}
