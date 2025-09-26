use crate::voxel::Voxel;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Clone)]
pub struct Chunk {
    data: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            data: vec![Voxel::AIR; CHUNK_VOLUME],
        }
    }

    #[inline]
    pub fn index(x: u32, y: u32, z: u32) -> usize {
        (x as usize) + (y as usize) * CHUNK_SIZE + (z as usize) * CHUNK_SIZE * CHUNK_SIZE
    }

    #[inline]
    pub fn in_bounds(x: u32, y: u32, z: u32) -> bool {
        x < CHUNK_SIZE as u32 && y < CHUNK_SIZE as u32 && z < CHUNK_SIZE as u32
    }

    pub fn get(&self, x: u32, y: u32, z: u32) -> Voxel {
        if !Self::in_bounds(x, y, z) {
            return Voxel::AIR;
        }
        self.data[Self::index(x, y, z)]
    }

    pub fn set(&mut self, x: u32, y: u32, z: u32, v: Voxel) {
        if !Self::in_bounds(x, y, z) {
            return;
        }
        let idx = Self::index(x, y, z);
        self.data[idx] = v;
    }
}
