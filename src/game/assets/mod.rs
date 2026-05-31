use bevy::prelude::*;

use crate::game::{
    assets::worldgen::{Biome, BiomeLoader, BiomeMapper, BiomeMapperLoader},
    atlas::{Atlas, AtlasLoader},
};

pub mod worldgen;

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
