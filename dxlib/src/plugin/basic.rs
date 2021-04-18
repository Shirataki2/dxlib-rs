use dxlib_sys::{dx_SetAlwaysRunFlag, dx_SetBackgroundColor};

use crate::{application::ApplicationBuilder, color::Color};

use super::Plugin;

pub struct BackgroundPlugin {
    pub run_always: bool,
    pub color: Color<u8>,
}

impl Plugin for BackgroundPlugin {
    fn build(&self, _app: &mut ApplicationBuilder) {
        unsafe {
            dx_SetAlwaysRunFlag(if self.run_always { 1 } else { 0 });
            dx_SetBackgroundColor(self.color.r as i32, self.color.g as i32, self.color.b as i32, self.color.a as i32);
        }
    }
}
