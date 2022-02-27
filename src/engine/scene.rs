use alloc::sync::Arc;
use alloc::vec::{IntoIter, Vec};
use crate::engine::{Entity, Voxel};

#[derive(Debug)]
pub struct Scene {
    pub(crate) entities: Vec<Arc<Entity>>
}

impl Scene {
    pub fn voxel_iter(&self) -> IntoIter<Arc<Voxel>> {
        let mut voxel_refs = vec![];
        for entity in &self.entities {
            for voxel in entity.as_ref().clone().into_iter() {
                voxel_refs.push(Arc::new(voxel));
            }
        }

        voxel_refs.into_iter()
    }
}
pub type EntityIter = IntoIter<Arc<Entity>>;

impl IntoIterator for Scene {
    type Item = Arc<Entity>;
    type IntoIter = EntityIter;

    fn into_iter(self) -> Self::IntoIter {
        self.entities.into_iter()
    }
}