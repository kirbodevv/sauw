use bevy::{math::Vec2, utils::default};
use bevy_rapier2d::prelude::Collider;

use crate::{
    constants::TILE_SIZE,
    game::{registry::Registry, world::block::*},
};

pub struct BlockDefinition {
    pub name: &'static str,
    pub texture: Option<&'static str>,
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
    pub fn new() -> Self {
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
                texture: Some("block/grass"),
                ..default()
            },
            "grass",
        );

        inner.insert(
            BlockDefinition {
                name: "tree",
                texture: Some("block/tree"),
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
                texture: Some("block/dirt"),
                ..default()
            },
            "dirt",
        );

        inner.insert(
            BlockDefinition {
                name: "flowers",
                texture: Some("block/flowers"),
                y_sort: 0.1,
                ..default()
            },
            "flowers",
        );

        inner.insert(
            BlockDefinition {
                name: "lily",
                texture: Some("block/lily"),
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
                texture: Some("block/sand"),
                ..default()
            },
            "sand",
        );

        inner.insert(
            BlockDefinition {
                name: "stone",
                texture: Some("block/stone"),
                ..default()
            },
            "stone",
        );

        inner.insert(
            BlockDefinition {
                name: "water",
                texture: Some("block/water"),
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
