use bevy::prelude::*;
use bevy_firefly::prelude::Occluder2d;
use bevy_rapier2d::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, CHUNK_VOLUME, TILE_SIZE},
    game::{
        GameState, ImageAssets,
        atlas::Atlas,
        registry::{
            GameRegistry,
            block_registry::{BlockDefinition, BlockId},
        },
        world::{
            BlockEntity, BlockPos, Chunk, ChunkCoord, WorldSeed, chunk_mesh::spawn_chunk_mesh,
        },
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    atlases: Res<Assets<Atlas>>,
    image_assets: Res<ImageAssets>,
) {
    let air = registry.blocks.id_by_name("air");
    for chunk in reader.read() {
        let chunk_world_x = chunk.chunk_coord.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE;
        let chunk_world_y = chunk.chunk_coord.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE;

        let mut chunk_entity = commands.spawn((
            Chunk,
            chunk.chunk_coord,
            Visibility::default(),
            Transform::from_xyz(chunk_world_x, chunk_world_y, 0.0),
        ));

        chunk_entity.with_children(|parent| {
            spawn_chunk_mesh(
                parent,
                &chunk.blocks,
                &registry,
                &image_assets.atlas_block_texture,
                &atlases.get(image_assets.atlas_block.id()).unwrap(),
                &mut meshes,
                &mut materials,
            );

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block = chunk.blocks[idx(x, y, 1)];
                    if block == air {
                        continue;
                    }
                    let block = registry.blocks.get(block);
                    spawn_block(parent, block, BlockPos::new(x as u8, y as u8, 1));
                }
            }
        });
    }
}

pub fn spawn_block(parent: &mut ChildSpawnerCommands<'_>, block: &BlockDefinition, pos: BlockPos) {
    let local_x = pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let local_y = pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let is_object = pos.layer == 1;

    if is_object {
        let mut entity = parent.spawn((
            Transform::from_xyz(local_x, local_y, pos.layer as f32),
            BlockEntity,
            RigidBody::Fixed,
            block.collider.clone(),
            pos.clone(),
        ));
        for occluder in &block.occluders {
            entity.with_children(|parent| {
                parent.spawn((
                    Occluder2d::rectangle(occluder.size.x, occluder.size.y),
                    Transform::from_xyz(
                        occluder.offset.x,
                        occluder.offset.y,
                        pos.layer as f32 * TILE_SIZE,
                    ),
                ));
            });
        }
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
