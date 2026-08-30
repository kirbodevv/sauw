use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::game::{
    registry::block_registry::BlockId,
    world::{
        ChunkCoord, chunk_positions,
        generator::{chunk_data, context::GenerationContext},
        types::{ChunkBlocks, idx_2d},
    },
};

pub fn generate_chunk(coord: ChunkCoord, ctx: &GenerationContext) -> ChunkBlocks {
    let mut blocks = ChunkBlocks::default();

    let chunk_data = chunk_data::generate(coord, &ctx.noise, &ctx.biome_mapper, &ctx.layer_mapper);

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

    blocks
}
