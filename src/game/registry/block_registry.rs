use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    constants::TILE_SIZE,
    game::{ImageAssets, registry::Registry},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<Handle<Image>>,
    pub sprite_size: Vec2,
    pub sprite_offset: Vec2,
    pub collider: Collider,
    pub y_sort: f32,
}

fn collider_with_offset(collider: Collider, collider_offset: Vec2) -> Collider {
    Collider::compound(vec![(collider_offset, 0.0, collider)])
}

impl Default for BlockDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            texture: None,
            sprite_size: Vec2::splat(TILE_SIZE),
            sprite_offset: Vec2::ZERO,
            collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            y_sort: 1.0,
        }
    }
}

pub struct BlockRegistry {
    inner: Registry<BlockDefinition>,
}

impl BlockRegistry {
    pub fn new(assets: &ImageAssets) -> Self {
        let mut inner = Registry::new();

        inner.insert(
            BlockDefinition {
                name: "air",
                ..default()
            },
            "air",
        );

        inner.insert(
            BlockDefinition {
                name: "grass",
                texture: Some(assets.block_grass.clone()),
                ..default()
            },
            "grass",
        );

        inner.insert(
            BlockDefinition {
                name: "tree",
                texture: Some(assets.block_tree.clone()),
                sprite_size: Vec2::new(32., 64.),
                sprite_offset: Vec2::new(0., 16.),
                collider: collider_with_offset(Collider::cuboid(5.0, 2.5), Vec2::new(0.0, -12.0)),
                ..default()
            },
            "tree",
        );

        inner.insert(
            BlockDefinition {
                name: "dirt",
                texture: Some(assets.block_dirt.clone()),
                ..default()
            },
            "dirt",
        );

        inner.insert(
            BlockDefinition {
                name: "flowers",
                texture: Some(assets.block_flowers.clone()),
                y_sort: 0.1,
                ..default()
            },
            "flowers",
        );

        inner.insert(
            BlockDefinition {
                name: "lily",
                texture: Some(assets.block_lily.clone()),
                y_sort: 0.1,
                sprite_size: Vec2::new(16., 16.),
                collider: Collider::cuboid(6.0, 6.0),
                ..default()
            },
            "lily",
        );

        inner.insert(
            BlockDefinition {
                name: "sand",
                texture: Some(assets.block_sand.clone()),
                ..default()
            },
            "sand",
        );

        inner.insert(
            BlockDefinition {
                name: "stone",
                texture: Some(assets.block_stone.clone()),
                ..default()
            },
            "stone",
        );

        inner.insert(
            BlockDefinition {
                name: "water",
                texture: Some(assets.block_water.clone()),
                ..default()
            },
            "water",
        );

        Self { inner }
    }

    #[allow(dead_code)]
    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        self.inner
            .get(id.0 as usize)
            .unwrap_or_else(|| panic!("Unknown BlockId {:?}", id))
    }

    pub fn by_name(&self, name: &str) -> &BlockDefinition {
        self.inner
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown Block {:?}", name))
    }

    #[allow(dead_code)]
    pub fn id_by_name(&self, name: &str) -> BlockId {
        BlockId(self.inner.id_by_name(name) as u16)
    }
}
