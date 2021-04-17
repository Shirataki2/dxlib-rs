use std::ptr;

use dxlib_sys::{
    consts::{
        DX_SCREEN_BACK, DX_SCREEN_FRONT, DX_SCREEN_OTHER, DX_SCREEN_TEMPFRONT, DX_SCREEN_WORK,
    },
    dx_ClearDrawScreen, dx_ScreenFlip, dx_SetDrawScreen,
};

use crate::error::{I32CodeExt, Result};

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum DrawScreen {
    Back = DX_SCREEN_BACK,
    Front = DX_SCREEN_FRONT,
    TempFront = DX_SCREEN_TEMPFRONT,
    Work = DX_SCREEN_WORK,
    Other = DX_SCREEN_OTHER,
}

#[derive(Clone, Copy, Debug)]
pub struct Screen {
    draw_screen: DrawScreen,
}

impl Default for Screen {
    fn default() -> Screen {
        Self {
            draw_screen: DrawScreen::Front,
        }
    }
}

impl Screen {
    pub fn get_draw_screen(&self) -> DrawScreen {
        self.draw_screen
    }

    pub fn set_draw_screen(&mut self, draw_screen: DrawScreen) -> Result<()> {
        self.draw_screen = draw_screen;
        unsafe { dx_SetDrawScreen(draw_screen as i32).ensure_zero()? };
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        unsafe { dx_ClearDrawScreen(ptr::null()).ensure_zero() }
    }

    pub fn flip(&self) -> Result<()> {
        unsafe { dx_ScreenFlip().ensure_zero() }
    }
}
