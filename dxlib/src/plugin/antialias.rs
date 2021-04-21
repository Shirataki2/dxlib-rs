use dxlib_sys::dx_SetFullSceneAntiAliasingMode;

use crate::{application::ApplicationBuilder, error::I32CodeExt, prelude::DxLibError};

use super::Plugin;

#[derive(Debug, Default)]
pub struct FullSceneAntiAliasPlugin {
    pub samples: u8,
    pub quality: u8,
}

impl Plugin for FullSceneAntiAliasPlugin {
    type Error = DxLibError;

    fn build(&self, _app: &mut ApplicationBuilder) -> Result<(), DxLibError> {
        unsafe {
            dx_SetFullSceneAntiAliasingMode(
                self.samples as i32,
                self.quality as i32,
            ).ensure_zero()?;
        }
        Ok(())
    }
}
