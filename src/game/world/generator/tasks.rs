use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{
        AsyncComputeTaskPool, Task,
        futures_lite::future::{block_on, poll_once},
    },
};

use crate::game::world::{
    ChunkBlocks, ChunkCoord,
    chunk_manager::ChunkBlocksStore,
    generator::{ChunkGenerateRequest, chunk::generate_chunk, context::GenerationContextHandle},
};

#[derive(Resource, Default)]
pub struct ChunkGenerationTasks {
    tasks: Vec<Task<(ChunkCoord, ChunkBlocks)>>,
}

impl ChunkGenerationTasks {
    pub fn poll(&mut self) -> Vec<(ChunkCoord, ChunkBlocks)> {
        let mut completed = Vec::new();

        self.tasks.retain_mut(|task| {
            if let Some(result) = block_on(poll_once(task)) {
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
            let blocks = generate_chunk(coord, &ctx);
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
