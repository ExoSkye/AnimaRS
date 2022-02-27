use cgmath::Vector3;
use num_traits::Zero;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Voxel {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl Voxel {
    pub(crate) fn new(pos: Option<Vector3<f32>>, rot: Option<Vector3<f32>>, scal: Option<Vector3<f32>>) -> Voxel {
        Voxel {
            position: pos.unwrap_or(Vector3::zero()),
            rotation: rot.unwrap_or(Vector3::zero()),
            scale: scal.unwrap_or(Vector3::zero()),
        }
    }
}