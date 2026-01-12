use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        GameState,
        registry::{
            GameRegistry,
            block_registry::{BlockDefinition, BlockId},
        },
        world::{BelongsToChunk, BlockEntity, BlockPos, ChunkCoord, WorldSeed, camera::YSort},
    },
};

#[inline]
pub fn idx(x: usize, y: usize, layer: usize) -> usize {
    x + y * CHUNK_SIZE + layer * CHUNK_SIZE * CHUNK_SIZE
}

#[derive(Message)]
pub struct ChunkGenerateRequest(pub ChunkCoord);

#[derive(Message)]
pub struct GeneratedChunk {
    pub chunk_coord: ChunkCoord,
    pub blocks: [BlockId; CHUNK_VOLUME],
}

pub fn generate_chunk(
    registry: Res<GameRegistry>,
    seed: Res<WorldSeed>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut writer: MessageWriter<GeneratedChunk>,
) {
    let width = CHUNK_SIZE as usize;
    let height = CHUNK_SIZE as usize;

    let perlin = Perlin::new(seed.0);

    let air = registry.blocks.id_by_name("air");
    let grass = registry.blocks.id_by_name("grass");
    let sand = registry.blocks.id_by_name("sand");
    let water = registry.blocks.id_by_name("water");
    let flowers = registry.blocks.id_by_name("lily");
    let tree = registry.blocks.id_by_name("tree");

    let freq = 0.1;

    for chunk in reader.read() {
        let mut blocks = [air; CHUNK_VOLUME];

        let chunk_coord = chunk.0;

        for x in 0..width {
            for y in 0..height {
                let chunk_x = chunk_coord.x as f64 * 16.0;
                let chunk_y = chunk_coord.y as f64 * 16.0;

                let noise_value =
                    perlin.get([(x as f64 + chunk_x) * freq, (y as f64 + chunk_y) * freq]);
                let normalized = (noise_value + 1.0) / 2.0;

                let surface = if normalized < 0.2 {
                    water
                } else if normalized < 0.3 {
                    sand
                } else {
                    grass
                };

                let mut top = air;

                if surface == grass {
                    let mut rng = rand::rng();
                    let r: f64 = rng.random();

                    if r < 0.05 {
                        top = tree;
                    } else if r < 0.15 {
                        top = flowers;
                    }
                }

                blocks[idx(x, y, 0)] = surface;
                blocks[idx(x, y, 1)] = top;
            }
        }

        writer.write(GeneratedChunk {
            chunk_coord,
            blocks,
        });
    }
}

pub fn spawn_chunk(
    registry: Res<GameRegistry>,
    mut commands: Commands,
    mut reader: MessageReader<GeneratedChunk>,
) {
    let air = registry.blocks.id_by_name("air");
    for chunk in reader.read() {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for layer in 0..2 as usize {
                    let block = chunk.blocks[idx(x, y, layer)];
                    if block == air {
                        continue;
                    }
                    let block = registry.blocks.get(block);
                    spawn_block(
                        &mut commands,
                        block,
                        chunk.chunk_coord,
                        BlockPos::new(x as u8, y as u8, layer as u8),
                    );
                }
            }
        }
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    block: &BlockDefinition,
    chunk_coord: ChunkCoord,
    pos: BlockPos,
) {
    let Some(texture) = &block.texture else {
        return;
    };

    let world_x =
        (chunk_coord.x * 16) as f32 * TILE_SIZE + pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y =
        (chunk_coord.y * 16) as f32 * TILE_SIZE + pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

    let y_sort = if pos.layer == 0 { 0.0 } else { block.y_sort };
    let y_sort = YSort { z: y_sort };

    let is_object = pos.layer == 1;

    let size = block.sprite_size;
    let mut entity = commands.spawn((
        Visibility::default(),
        Transform::from_xyz(world_x, world_y, pos.layer as f32),
        BlockEntity,
        BelongsToChunk(chunk_coord),
        pos.clone(),
        y_sort,
    ));

    entity.with_children(|parent| {
        parent.spawn((
            Sprite {
                image: texture.clone(),
                custom_size: Some(size),
                ..default()
            },
            Transform::from_xyz(
                block.sprite_offset.x,
                block.sprite_offset.y,
                pos.layer as f32 * TILE_SIZE,
            ),
        ));
    });

    if is_object {
        entity.insert((RigidBody::Fixed, block.collider.clone()));
    }
}

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChunkGenerateRequest>()
            .add_message::<GeneratedChunk>()
            .add_systems(
                Update,
                (generate_chunk, spawn_chunk)
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
