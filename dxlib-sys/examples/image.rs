use dxlib_sys::*;
use std::{process::exit, ptr};

const INIT_TITLE: &'static [u8] = &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00];

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(640, 480, 32, 60);
        dx_SetMainWindowText(INIT_TITLE.as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }

        let handle = dx_LoadGraph("lena.png".as_ptr() as *const i8, FALSE);

        while dx_ProcessMessage() == 0 {
            dx_ClearDrawScreen(ptr::null());
            dx_SetDrawScreen(DX_SCREEN_BACK);

            dx_DrawGraph(0, 0, handle, 0);

            dx_ScreenFlip();
        }

        dx_DxLib_End();
    }
    Ok(())
}
