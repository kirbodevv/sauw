use crate::game::world::block::{BlockId, Layer};

const CHUNK_SIZE: usize = 16;

#[derive(Clone)]
pub struct Chunk {
    ground: [[BlockId; CHUNK_SIZE]; CHUNK_SIZE],
    objects: [[BlockId; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ground: [[BlockId(1); CHUNK_SIZE]; CHUNK_SIZE],
            objects: [[BlockId(0); CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn get(&self, x: usize, y: usize, layer: Layer) -> BlockId {
        match layer {
            Layer::Ground => self.ground[y][x],
            Layer::Object => self.objects[y][x],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, layer: Layer, id: BlockId) {
        match layer {
            Layer::Ground => self.ground[y][x] = id,
            Layer::Object => self.objects[y][x] = id,
        }
    }
}
