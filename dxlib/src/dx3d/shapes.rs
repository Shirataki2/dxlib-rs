use crate::{math::vector::Vector3, color::Color, error::{I32CodeExt, Result}};
use dxlib_sys::dx_DrawLine3D;
use smart_default::SmartDefault;

pub enum Shape3D {
    Line(Line3D)
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Line3D {
    pub start: Vector3<f32>,
    #[default(_code = "Vector3::from([1.0, 0.0, 0.0])")]
    pub end: Vector3<f32>,
    pub color: Color<u8>,
}

impl Line3D {
    pub fn draw(&self) -> Result<()> {
        unsafe { dx_DrawLine3D(self.start.into(), self.end.into(), self.color.as_u32()).ensure_zero() }
    }
}
