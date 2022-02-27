use alloc::vec::{IntoIter, Vec};
use crate::engine::Voxel;

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub(crate) voxels: Vec<Voxel>
}

impl Entity {}

pub type VoxelIter = IntoIter<Voxel>;

impl IntoIterator for Entity {
    type Item = Voxel;
    type IntoIter = VoxelIter;

    fn into_iter(self) -> Self::IntoIter {
        self.voxels.into_iter()
    }
}