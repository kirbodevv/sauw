use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};

use serde::Deserialize;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct Recipe {
    #[serde(skip)]
    #[serde(default)]
    pub name: String,
    pub result: String,
    pub count: u32,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Deserialize)]
pub struct Ingredient {
    pub item: String,
    pub count: i32,
}

#[derive(Default, TypePath)]
pub struct RecipeLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RecipeLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for RecipeLoader {
    type Asset = Recipe;
    type Settings = ();
    type Error = RecipeLoaderError;

    fn extensions(&self) -> &[&str] {
        &["recipe"]
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let path = load_context.path();
        let file_name = path
            .path()
            .with_extension("")
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();

        let mut recipe: Recipe = serde_json::from_slice(&bytes)
            .map_err(|e| RecipeLoaderError::Io(IoError::new(ErrorKind::Other, e)))?;

        recipe.name = file_name;

        info!(
            target: "asset_loader",
            "Loaded recipe: {}",
            recipe.result
        );

        Ok(recipe)
    }
}
