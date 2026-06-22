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
        app.init_asset::<atlas::Atlas>()
            .init_asset::<worldgen::Biome>()
            .init_asset::<worldgen::BiomeMapper>()
            .init_asset::<worldgen::LayerMapper>()
            .init_asset::<recipe::Recipe>()
            .init_asset_loader::<atlas::AtlasLoader>()
            .init_asset_loader::<worldgen::BiomeMapperLoader>()
            .init_asset_loader::<worldgen::BiomeLoader>()
            .init_asset_loader::<worldgen::LayerMapperLoader>()
            .init_asset_loader::<recipe::RecipeLoader>()
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
