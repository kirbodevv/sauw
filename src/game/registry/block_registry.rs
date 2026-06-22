use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{constants::TILE_SIZE, game::registry::Registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

pub struct BlockDefinition {
    pub name: &'static str,
    pub sprite_size: Vec2,
    pub sprite_offset: Vec2,
    pub collider: Collider,
    pub occluders: Vec<Occluder>,
    pub y_sort: f32,
}

pub struct Occluder {
    pub size: Vec2,
    pub offset: Vec2,
}

impl Occluder {
    pub fn new(half_size: Vec2, offset: Vec2) -> Self {
        Self {
            size: half_size,
            offset,
        }
    }
}

fn collider_with_offset(collider: Collider, collider_offset: Vec2) -> Collider {
    Collider::compound(vec![(collider_offset, 0.0, collider)])
}

impl Default for BlockDefinition {
    fn default() -> Self {
        Self {
            name: "none",
            sprite_size: Vec2::splat(TILE_SIZE),
            sprite_offset: Vec2::ZERO,
            collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            occluders: vec![Occluder::default()],
            y_sort: 1.0,
        }
    }
}

impl Default for Occluder {
    fn default() -> Self {
        Self {
            size: Vec2::splat(TILE_SIZE),
            offset: Vec2::ZERO,
        }
    }
}

#[derive(Resource)]
pub struct BlockRegistry {
    inner: Registry<BlockDefinition>,
}

impl BlockRegistry {
    #[allow(dead_code)]
    pub fn get(&self, id: BlockId) -> &BlockDefinition {
        self.inner
            .get(id.0 as usize)
            .unwrap_or_else(|| panic!("Unknown BlockId {:?}", id))
    }

    #[allow(dead_code)]
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

pub fn init_blocks(mut commands: Commands) {
    let mut inner = Registry::new("block");

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
            ..default()
        },
        "grass",
    );

    inner.insert(
        BlockDefinition {
            name: "tree",
            sprite_size: Vec2::new(32., 64.),
            sprite_offset: Vec2::new(0., 16.),
            collider: collider_with_offset(Collider::cuboid(5.0, 2.5), Vec2::new(0.0, -12.0)),
            occluders: vec![Occluder::new(Vec2::new(6.0, 6.0), Vec2::new(0.0, -12.0))],
            ..default()
        },
        "tree",
    );

    inner.insert(
        BlockDefinition {
            name: "dirt",
            ..default()
        },
        "dirt",
    );

    inner.insert(
        BlockDefinition {
            name: "flowers",
            y_sort: 0.1,
            ..default()
        },
        "flowers",
    );

    inner.insert(
        BlockDefinition {
            name: "lily",
            y_sort: 0.1,
            sprite_size: Vec2::new(16., 16.),
            collider: Collider::cuboid(6.0, 6.0),
            occluders: vec![],
            ..default()
        },
        "lily",
    );

    inner.insert(
        BlockDefinition {
            name: "sand",
            ..default()
        },
        "sand",
    );

    inner.insert(
        BlockDefinition {
            name: "stone",
            ..default()
        },
        "stone",
    );

    inner.insert(
        BlockDefinition {
            name: "water",
            ..default()
        },
        "water",
    );

    inner.insert(
        BlockDefinition {
            name: "cactus",
            ..default()
        },
        "cactus",
    );

    let blocks = BlockRegistry { inner };

    commands.insert_resource(blocks);
}
