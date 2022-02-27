#![no_std]

#[macro_use]
extern crate alloc;

pub mod common;
pub mod engine;
pub mod backends;
pub mod dummy_backend;

#[cfg(test)]
mod tests {

    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    use alloc::sync::Arc;
    use cgmath::Vector3;
    use crate::engine::{Entity, Scene, Voxel};

    #[test]
    fn scene_iter_works() {
        let entity = Entity {
            voxels: vec![]
        };

        let entity_arc = Arc::new(entity);

        let scene = Scene {
            entities: vec![entity_arc.clone()]
        };

        assert_eq!(scene.into_iter().next().unwrap(), entity_arc);
    }

    #[test]
    fn voxel_iter_works() {
        let voxel1 = Voxel::new(Some(
            Vector3{
                x: 1.4,
                y: 0.0,
                z: 0.0
            }), None, None
        );

        let voxel2 = Voxel::new(None, None, None);

        let voxel3 = Voxel::new(Some(
            Vector3{
                x: 1.1,
                y: 0.0,
                z: 0.0
            }), None, None
        );

        let voxel4 = Voxel::new(None, None, None);

        let entity1 = Entity {
            voxels: vec![voxel1, voxel2]
        };

        let entity2 = Entity {
            voxels: vec![voxel3, voxel4]
        };

        let scene = Scene {
            entities: vec![Arc::new(entity1), Arc::new(entity2)]
        };

        let mut voxel_iter = scene.voxel_iter();

        assert_eq!(voxel_iter.next().unwrap().position, voxel1.position);
        assert_eq!(voxel_iter.next().unwrap().position, voxel2.position);
        assert_eq!(voxel_iter.next().unwrap().position, voxel3.position);
        assert_eq!(voxel_iter.next().unwrap().position, voxel4.position);
    }

}
