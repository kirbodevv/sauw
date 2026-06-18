pub mod atlas;
pub mod recipe;
pub mod worldgen;

use bevy::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<atlas::Atlas>()
            .init_asset::<worldgen::Biome>()
            .init_asset::<worldgen::BiomeMapper>()
            .init_asset::<worldgen::LayerMapper>()
            .init_asset::<recipe::Recipe>()
            .init_asset_loader::<atlas::AtlasLoader>()
            .init_asset_loader::<worldgen::BiomeMapperLoader>()
            .init_asset_loader::<worldgen::BiomeLoader>()
            .init_asset_loader::<worldgen::LayerMapperLoader>()
            .init_asset_loader::<recipe::RecipeLoader>();
    }
}
