use core::array::IntoIter;
use core::option::Option;
use core::result::Result;
use num_traits::real::Real;
use cgmath::Vector2;
use crate::backends::ExitType;
use crate::engine::voxel::Voxel;

pub enum GraphicsType {
    END,
    G1,
    G15,
    G2,
    G3,
    G4,
    G5,
    GX
}

pub enum VideoError {
    BadStandard,
    BadResolution
}

pub trait VideoBackend {
    fn get_supported_graphics_standards() -> IntoIter<GraphicsType, 8>;

    #[must_use]
    fn initialize<T: Real>(resolution: Option<Vector2<T>>) -> Result<(), VideoError>;

    #[must_use]
    fn set_standard(standard: GraphicsType) -> Result<(), VideoError>;

    #[must_use]
    fn draw<const N: usize>(voxels: IntoIter<Voxel, N>) -> Result<(), VideoError>;

    #[must_use]
    fn flip() -> Result<(), VideoError>;

    // TODO: UI

    #[must_use]
    fn exit(exit_type: ExitType) -> Result<(), VideoError>;
}
