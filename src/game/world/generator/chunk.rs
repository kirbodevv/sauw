use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{
        AsyncComputeTaskPool, Task,
        futures_lite::{self},
    },
};
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::game::{
    registry::block_registry::BlockId,
    world::{
        ChunkCoord,
        chunk_manager::ChunkBlocksStore,
        chunk_positions,
        generator::{ChunkGenerateRequest, chunk_data, context::GenerationContextHandle},
        types::{ChunkBlocks, idx_2d},
    },
};

#[derive(Resource, Default)]
pub struct ChunkGenerationTasks {
    tasks: Vec<Task<(ChunkCoord, ChunkBlocks)>>,
}

impl ChunkGenerationTasks {
    pub fn poll(&mut self) -> Vec<(ChunkCoord, ChunkBlocks)> {
        let mut completed = Vec::new();

        self.tasks.retain_mut(|task| {
            if let Some(result) =
                futures_lite::future::block_on(futures_lite::future::poll_once(task))
            {
                completed.push(result);
                false
            } else {
                true
            }
        });

        completed
    }

    pub fn add(&mut self, task: Task<(ChunkCoord, ChunkBlocks)>) {
        self.tasks.push(task);
    }
}

pub fn spawn_generation_tasks(
    ctx: Res<GenerationContextHandle>,
    mut reader: MessageReader<ChunkGenerateRequest>,
    mut tasks: ResMut<ChunkGenerationTasks>,
) {
    let pool = AsyncComputeTaskPool::get();

    for request in reader.read() {
        let coord = request.0;
        let ctx = Arc::clone(&ctx.0);

        let task = pool.spawn(async move {
            let mut blocks = ChunkBlocks::default();

            let chunk_data =
                chunk_data::generate(coord, &ctx.noise, &ctx.biome_mapper, &ctx.layer_mapper);

            let seed = ctx.noise.settings.seed.0 as u64;
            let x = coord.x as i64 as u64;
            let y = coord.y as i64 as u64;

            let mut objects_rng = SmallRng::seed_from_u64(seed.wrapping_add(x).wrapping_add(y));

            for (x, y) in chunk_positions() {
                let index = idx_2d(x, y);
                let biome = ctx.biomes.by_id(chunk_data.biome_map[index]);

                let mut top = BlockId::AIR;

                if let Some(objects) = &biome.objects {
                    let r: f32 = objects_rng.random();

                    let mut cumulative = 0.0;

                    for object in objects {
                        cumulative += object.chance;

                        if r < cumulative {
                            top = object.block;
                            break;
                        }
                    }
                }

                blocks.ground[index] = biome.surface;
                blocks.objects[index] = top;
            }

            (coord, blocks)
        });

        tasks.add(task);
    }
}

pub fn poll_generation_tasks(
    mut tasks: ResMut<ChunkGenerationTasks>,
    mut store: ResMut<ChunkBlocksStore>,
) {
    for (coord, blocks) in tasks.poll() {
        store.blocks.insert(coord, blocks);
    }
}
