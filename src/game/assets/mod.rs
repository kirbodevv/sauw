pub mod atlas;
pub mod recipe;
pub mod resource;
pub mod worldgen;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::GameState;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<atlas::AtlasAsset>()
            .init_asset::<worldgen::BiomeAsset>()
            .init_asset::<worldgen::BiomeMapperAsset>()
            .init_asset::<worldgen::LayerMapperAsset>()
            .init_asset::<recipe::RecipeAsset>()
            .init_asset_loader::<atlas::AtlasAssetLoader>()
            .init_asset_loader::<worldgen::BiomeMapperAssetLoader>()
            .init_asset_loader::<worldgen::BiomeAssetLoader>()
            .init_asset_loader::<worldgen::LayerMapperAssetLoader>()
            .init_asset_loader::<recipe::RecipeAssetLoader>()
            .add_loading_state(
                LoadingState::new(GameState::AssetsLoading)
                    .continue_to_state(GameState::Bootstrap)
                    .load_collection::<resource::ImageAssets>()
                    .load_collection::<resource::AtlasAssets>()
                    .load_collection::<resource::WorldgenAssets>()
                    .load_collection::<resource::RecipeAssets>(),
            );
    }
}
