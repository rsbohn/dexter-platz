#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Voxel(pub u8);

impl Voxel {
    pub const AIR: Voxel = Voxel(0);
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}
