use dxlib_sys::dx_SetGlobalAmbientLight;

use crate::{
    color::Color,
    error::{I32CodeExt, Result},
};

pub struct World;

impl World {
    pub fn set_ambient(ambient: Color<f32>) -> Result<()> {
        unsafe { dx_SetGlobalAmbientLight(ambient.into()).ensure_zero() }
    }
}
