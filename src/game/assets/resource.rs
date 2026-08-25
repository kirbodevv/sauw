use crate::game::assets::{
    atlas::AtlasAsset,
    recipe::RecipeAsset,
    worldgen::{BiomeAsset, BiomeMapperAsset, LayerMapperAsset, NoiseSettingsAsset},
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(SystemParam)]
pub struct AtlasAssetsParam<'w> {
    atlases: Res<'w, Assets<AtlasAsset>>,
    image_assets: Res<'w, ImageAssets>,
    atlas_assets: Res<'w, AtlasAssets>,
}

impl AtlasAssetsParam<'_> {
    pub fn block_texture(&self) -> Handle<Image> {
        self.image_assets.block.clone()
    }

    pub fn block_normal_texture(&self) -> Handle<Image> {
        self.image_assets.block_normal.clone()
    }

    pub fn block_atlas(&self) -> &AtlasAsset {
        self.atlases.get(self.atlas_assets.block.id()).unwrap()
    }

    pub fn item_texture(&self) -> Handle<Image> {
        self.image_assets.item.clone()
    }

    pub fn item_atlas(&self) -> &AtlasAsset {
        self.atlases.get(self.atlas_assets.item.id()).unwrap()
    }
}

#[derive(AssetCollection, Resource)]
pub struct AtlasAssets {
    #[asset(path = "atlas/block.json")]
    pub block: Handle<AtlasAsset>,

    #[asset(path = "atlas/item.json")]
    pub item: Handle<AtlasAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "atlas/block.png")]
    pub block: Handle<Image>,

    #[asset(path = "atlas/block_normal.png")]
    pub block_normal: Handle<Image>,

    #[asset(path = "atlas/item.png")]
    pub item: Handle<Image>,

    #[asset(path = "entity/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "ui/hud/heart_full.png")]
    pub ui_heart_full: Handle<Image>,

    #[asset(path = "ui/hud/heart_empty.png")]
    pub ui_heart_empty: Handle<Image>,

    #[asset(path = "ui/hud/inventory.png")]
    pub ui_inventory: Handle<Image>,

    #[asset(path = "ui/hud/selected_slot.png")]
    pub ui_selected_slot: Handle<Image>,

    #[asset(path = "ui/hud/joystick_handle.png")]
    pub ui_joystick_handle: Handle<Image>,

    #[asset(path = "ui/hud/joystick_base.png")]
    pub ui_joystick_base: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct WorldgenMapperAssets {
    #[asset(path = "worldgen/layer.lmap")]
    pub layer_mapper: Handle<LayerMapperAsset>,

    #[asset(path = "worldgen/biome.bmap")]
    pub biome_mapper: Handle<BiomeMapperAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct BiomeAssets {
    #[asset(path = "worldgen/biome", collection(typed))]
    pub biomes: Vec<Handle<BiomeAsset>>,
}

#[derive(AssetCollection, Resource)]
pub struct RecipeAssets {
    #[asset(path = "recipes", collection(typed))]
    pub recipes: Vec<Handle<RecipeAsset>>,
}

#[derive(AssetCollection, Resource)]
pub struct NoiseSettingsAssets {
    #[asset(path = "worldgen/settings.noise")]
    pub noise_settings: Handle<NoiseSettingsAsset>,
}
