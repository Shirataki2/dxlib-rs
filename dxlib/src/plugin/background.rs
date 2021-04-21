use dxlib_sys::{dx_SetAlwaysRunFlag, dx_SetBackgroundColor};

use crate::{
    application::ApplicationBuilder,
    color::Color,
    error::{DxLibError, I32CodeExt},
};

use super::Plugin;

#[derive(Debug, Default)]
pub struct BackgroundPlugin {
    pub run_always: bool,
    pub color: Color<u8>,
}

impl Plugin for BackgroundPlugin {
    type Error = DxLibError;
    fn build(&self, _app: &mut ApplicationBuilder) -> Result<(), DxLibError> {
        unsafe {
            dx_SetAlwaysRunFlag(if self.run_always { 1 } else { 0 }).ensure_zero()?;
            dx_SetBackgroundColor(
                self.color.r as i32,
                self.color.g as i32,
                self.color.b as i32,
                self.color.a as i32,
            )
            .ensure_zero()?;
        }
        Ok(())
    }
}
