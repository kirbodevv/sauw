use std::collections::HashSet;

use bevy::prelude::*;

use crate::game::{
    player::CurrentPlayerChunk,
    world::{ChunkCoord, config::WorldConfig},
};

use super::RequiredChunks;

pub(super) fn compute_required_chunks(
    mut required_chunks: ResMut<RequiredChunks>,
    player_chunk: Res<CurrentPlayerChunk>,
    config: Res<WorldConfig>,
) {
    if !player_chunk.is_changed() {
        return;
    }

    let Some(player_chunk) = player_chunk.0 else {
        return;
    };

    let load_radius = config.load_radius;

    let mut required = HashSet::new();
    for cx in (player_chunk.x - load_radius)..=(player_chunk.x + load_radius) {
        for cy in (player_chunk.y - load_radius)..=(player_chunk.y + load_radius) {
            required.insert(ChunkCoord { x: cx, y: cy });
        }
    }

    let mut ordered: Vec<ChunkCoord> = required.iter().copied().collect();
    ordered.sort_by_key(|coord| {
        let dx = (coord.x - player_chunk.x) as i64;
        let dy = (coord.y - player_chunk.y) as i64;
        dx * dx + dy * dy
    });

    required_chunks.set = required;
    required_chunks.ordered = ordered;
}
