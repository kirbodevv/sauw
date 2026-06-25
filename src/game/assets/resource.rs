use crate::game::assets::{
    atlas::AtlasAsset,
    recipe::RecipeAsset,
    worldgen::{BiomeAsset, BiomeMapperAsset, LayerMapperAsset},
};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

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

    #[asset(path = "atlas/item.png")]
    pub item: Handle<Image>,

    #[asset(path = "entity/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "ui/heart_full.png")]
    pub ui_heart_full: Handle<Image>,

    #[asset(path = "ui/heart_empty.png")]
    pub ui_heart_empty: Handle<Image>,

    #[asset(path = "ui/inventory.png")]
    pub ui_inventory: Handle<Image>,

    #[asset(path = "ui/selected_slot.png")]
    pub ui_selected_slot: Handle<Image>,

    #[asset(path = "ui/joystick_handle.png")]
    pub ui_joystick_handle: Handle<Image>,

    #[asset(path = "ui/joystick_base.png")]
    pub ui_joystick_base: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct WorldgenAssets {
    #[asset(path = "worldgen/layer.lmap")]
    pub layer_mapper: Handle<LayerMapperAsset>,

    #[asset(path = "worldgen/biome.bmap")]
    pub biome_mapper: Handle<BiomeMapperAsset>,

    #[asset(path = "worldgen/biome/beach.biome")]
    pub beach_biome: Handle<BiomeAsset>,

    #[asset(path = "worldgen/biome/desert.biome")]
    pub desert_biome: Handle<BiomeAsset>,

    #[asset(path = "worldgen/biome/forest.biome")]
    pub forest_biome: Handle<BiomeAsset>,

    #[asset(path = "worldgen/biome/ocean.biome")]
    pub ocean_biome: Handle<BiomeAsset>,

    #[asset(path = "worldgen/biome/plains.biome")]
    pub plains_biome: Handle<BiomeAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct RecipeAssets {
    #[asset(path = "recipes/rope.recipe")]
    pub rope: Handle<RecipeAsset>,

    #[asset(path = "recipes/stone_axe.recipe")]
    pub stone_axe: Handle<RecipeAsset>,

    #[asset(path = "recipes/stone_pickaxe.recipe")]
    pub stone_pickaxe: Handle<RecipeAsset>,

    #[asset(path = "recipes/stone_shovel.recipe")]
    pub stone_shovel: Handle<RecipeAsset>,
}
