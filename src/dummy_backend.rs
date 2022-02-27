use core::array::IntoIter;
use cgmath::Vector2;
use num_traits::real::Real;
use crate::backends::ExitType;
use crate::backends::video::{GraphicsType, VideoBackend, VideoError};
use crate::backends::io::IOBackend;
use crate::backends::video::GraphicsType::{G1, G15, G2, G3, G4, G5, GX, END};
use crate::engine::Voxel;

struct DummyBackend;

impl VideoBackend for DummyBackend {
    fn get_supported_graphics_standards() -> IntoIter<GraphicsType, 8> {
        [G1, G15, G2, G3, G4, G5, GX, END].into_iter()
    }

    fn initialize<T: Real>(_resolution: Option<Vector2<T>>) -> Result<(), VideoError> {
        Ok(())
    }

    fn set_standard(_standard: GraphicsType) -> Result<(), VideoError> {
        Ok(())
    }

    fn draw<T: IntoIterator<Item = Voxel>, const N: usize>(voxels: &T) -> Result<(), VideoError> {
        Ok(())
    }

    fn flip() -> Result<(), VideoError> {
        Ok(())
    }

    fn exit(_exit_type: ExitType) -> Result<(), VideoError> {
        Ok(())
    }
}

impl IOBackend for DummyBackend {

}