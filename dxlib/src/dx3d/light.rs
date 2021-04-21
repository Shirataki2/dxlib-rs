use dxlib_sys::dx_SetGlobalAmbientLight;

use crate::{error::{Result, I32CodeExt}, color::Color};

pub struct World;

impl World {
    pub fn set_ambient(ambient: Color<f32>) -> Result<()> {
        unsafe { dx_SetGlobalAmbientLight(ambient.into()).ensure_zero() }
    }
}