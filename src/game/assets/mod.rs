pub mod atlas;
pub mod worldgen;

use crate::game::assets::{
    atlas::{Atlas, AtlasLoader},
    worldgen::{
        Biome, BiomeLoader, BiomeMapper, BiomeMapperLoader, LayerMapper, LayerMapperLoader,
    },
};
use bevy::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Atlas>()
            .init_asset::<Biome>()
            .init_asset::<BiomeMapper>()
            .init_asset::<LayerMapper>()
            .init_asset_loader::<AtlasLoader>()
            .init_asset_loader::<BiomeMapperLoader>()
            .init_asset_loader::<BiomeLoader>()
            .init_asset_loader::<LayerMapperLoader>();
    }
}
